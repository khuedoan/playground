import {copyFile, mkdir} from "node:fs/promises"
import {chromium} from "playwright"

const controlUrl = process.env.CONTROL_URL || "http://127.0.0.1:4000"
const artifacts = process.env.ARTIFACTS_DIR || "/artifacts"
const bootTimeout = Number(process.env.DEMO_BOOT_TIMEOUT_MS || "900000")
const agentTimeout = Number(process.env.DEMO_AGENT_TIMEOUT_MS || "600000")
const prompt = process.env.DEMO_PROMPT ||
  "Use the bash tool to run exactly: date -u +%FT%TZ > /workspace/workbench-demo.txt && test -s /workspace/workbench-demo.txt. Do not only show me the command. After it succeeds, reply exactly: Created and verified workbench-demo.txt"

async function waitForRenderedDesktop(card) {
  const iframe = card.locator('iframe[title$=" desktop"]')
  await iframe.waitFor({state: "visible", timeout: 60_000})

  const iframeHandle = await iframe.elementHandle()
  const frame = await iframeHandle.contentFrame()
  if (!frame) {
    throw new Error("Wayland desktop iframe did not create a browser frame")
  }

  // The iframe can connect while wayvnc is still publishing its first frame.
  // Reconnect after the workspace is running, then require real dark pixels from
  // the terminal instead of accepting noVNC's blank white canvas.
  await frame.goto(frame.url(), {waitUntil: "domcontentloaded", timeout: 60_000})
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
  await page.getByText("Your private network").waitFor()
  await page.waitForTimeout(1_500)

  const title = `MicroVM demo ${new Date().toISOString().slice(11, 19)}`
  await page.getByPlaceholder("e.g. Inspect customer dataset").fill(title)
  await page.getByRole("button", {name: "Launch"}).click()

  const card = page.locator("article.workspace-card").filter({hasText: title})
  const running = card.getByText("running", {exact: true})
  const failed = card.getByText("failed", {exact: true})
  await Promise.race([
    running.waitFor({timeout: bootTimeout}),
    failed.waitFor({timeout: bootTimeout}).then(async () => {
      throw new Error((await card.textContent()) || "MicroVM provisioning failed")
    }),
  ])
  await page.waitForTimeout(3_000)
  await waitForRenderedDesktop(card)

  await card.getByPlaceholder("Ask Pi to do something in this workspace…").fill(prompt)
  await card.getByRole("button", {name: "Send"}).click()
  const assistant = card.locator(".message-assistant")
  const agentError = card.locator(".message-error")
  await Promise.race([
    assistant.waitFor({timeout: agentTimeout}),
    agentError.waitFor({timeout: agentTimeout}).then(async () => {
      throw new Error((await agentError.textContent()) || "Pi failed without an error message")
    }),
  ])
  await page.waitForTimeout(4_000)
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
