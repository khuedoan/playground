import {copyFile, mkdir} from "node:fs/promises"
import {chromium} from "playwright"

const controlUrl = process.env.CONTROL_URL || "http://127.0.0.1:4000"
const artifacts = process.env.ARTIFACTS_DIR || "/artifacts"
const bootTimeout = Number(process.env.DEMO_BOOT_TIMEOUT_MS || "900000")
const agentTimeout = Number(process.env.DEMO_AGENT_TIMEOUT_MS || "600000")
const prompt = process.env.DEMO_PROMPT ||
  "Use the bash tool to run exactly: date -u +%FT%TZ > /workspace/workbench-demo.txt && test -s /workspace/workbench-demo.txt. Do not only show me the command. After it succeeds, reply exactly: Created and verified workbench-demo.txt"

async function waitForRenderedDesktop(page) {
  const iframe = page.locator('.workspace-inspector iframe[title$=" desktop"]')
  await iframe.waitFor({state: "visible", timeout: 60_000})

  const iframeHandle = await iframe.elementHandle()
  const frame = await iframeHandle.contentFrame()
  if (!frame) {
    throw new Error("Wayland desktop iframe did not create a browser frame")
  }
  const desktopUrl = await iframe.getAttribute("src")
  if (!desktopUrl) {
    throw new Error("Wayland desktop iframe has no source URL")
  }

  // The iframe can connect while wayvnc is still publishing its first frame.
  // Reconnect after the workspace is running, then require real dark pixels from
  // the terminal instead of accepting noVNC's blank white canvas.
  let navigationError
  for (let attempt = 0; attempt < 12; attempt += 1) {
    try {
      const response = await frame.goto(desktopUrl, {
        waitUntil: "domcontentloaded",
        timeout: 10_000,
      })
      if (response && !response.ok()) {
        throw new Error(`noVNC returned HTTP ${response.status()}`)
      }
      navigationError = undefined
      break
    } catch (error) {
      navigationError = error
      await new Promise(resolve => setTimeout(resolve, 1_000))
    }
  }
  if (navigationError) {
    throw new Error(`noVNC did not become reachable: ${navigationError.message}`)
  }
  await frame.locator("#noVNC_container canvas").waitFor({state: "visible", timeout: 60_000})
  await frame.waitForFunction(() => {
    const canvas = document.querySelector("#noVNC_container canvas")
    if (!(canvas instanceof HTMLCanvasElement) || canvas.width < 640 || canvas.height < 360) {
      return false
    }

    const context = canvas.getContext("2d", {willReadFrequently: true})
    if (!context) return false

    let dark = 0
    let sampled = 0
    const stepX = Math.max(1, Math.floor(canvas.width / 24))
    const stepY = Math.max(1, Math.floor(canvas.height / 14))
    for (let y = Math.floor(stepY / 2); y < canvas.height; y += stepY) {
      for (let x = Math.floor(stepX / 2); x < canvas.width; x += stepX) {
        const pixel = context.getImageData(x, y, 1, 1).data
        const luminance = (pixel[0] + pixel[1] + pixel[2]) / 3
        dark += pixel[3] > 0 && luminance < 180 ? 1 : 0
        sampled += 1
      }
    }
    return dark / sampled > 0.5
  }, null, {polling: 500, timeout: 60_000})
}

await mkdir(artifacts, {recursive: true})
const browser = await chromium.launch({headless: true})
const context = await browser.newContext({
  viewport: {width: 1440, height: 900},
  recordVideo: {dir: artifacts, size: {width: 1440, height: 900}},
})
const page = await context.newPage()
const video = page.video()

try {
  await page.goto(controlUrl, {waitUntil: "networkidle", timeout: 60_000})
  await page.getByText("Warm pool online").waitFor()
  await page.waitForTimeout(1_000)

  const suffix = new Date().toISOString().slice(11, 19)
  const primaryTitle = `MicroVM demo ${suffix}`
  const parallelTitle = `Parallel review ${suffix}`
  const titleInput = page.getByPlaceholder("Name a new thread")
  await titleInput.fill(primaryTitle)
  await page.getByRole("button", {name: "Start new thread"}).click()
  await page.getByRole("button", {name: new RegExp(primaryTitle)}).waitFor()
  await titleInput.fill(parallelTitle)
  await page.getByRole("button", {name: "Start new thread"}).click()

  const primaryThread = page.getByRole("button", {name: new RegExp(primaryTitle)})
  const parallelThread = page.getByRole("button", {name: new RegExp(parallelTitle)})
  await parallelThread.waitFor()

  const waitUntilRunning = async thread => {
    await Promise.race([
      thread.locator(".thread-status-running").waitFor({timeout: bootTimeout}),
      thread.locator(".thread-status-failed").waitFor({timeout: bootTimeout}).then(async () => {
        throw new Error((await thread.textContent()) || "MicroVM provisioning failed")
      }),
    ])
  }
  await Promise.all([waitUntilRunning(primaryThread), waitUntilRunning(parallelThread)])
  await primaryThread.click()
  await page.waitForTimeout(1_000)
  await waitForRenderedDesktop(page)

  await page.getByPlaceholder("Ask Workbench to make a change…").fill(prompt)
  await page.getByRole("button", {name: "Send message"}).click()
  const assistant = page.locator(".thread-main .message-assistant")
  const agentError = page.locator(".thread-main .message-error")
  await Promise.race([
    assistant.waitFor({timeout: agentTimeout}),
    agentError.waitFor({timeout: agentTimeout}).then(async () => {
      throw new Error((await agentError.textContent()) || "Pi failed without an error message")
    }),
  ])
  await page.waitForTimeout(2_000)
  if (await page.locator(".thread-item").count() !== 2) {
    throw new Error("expected both parallel agent threads to remain visible")
  }
  await page.screenshot({path: `${artifacts}/workbench-demo-final.png`, fullPage: true})
} finally {
  await page.close()
  await context.close()
  await browser.close()
}

if (video) {
  const rawVideo = await video.path()
  await copyFile(rawVideo, `${artifacts}/workbench-demo.webm`)
  process.stdout.write(`Recorded real browser session: ${artifacts}/workbench-demo.webm\n`)
}
