import {copyFile, mkdir} from "node:fs/promises"
import {chromium} from "playwright"

const controlUrl = process.env.CONTROL_URL || "http://127.0.0.1:4000"
const artifacts = process.env.ARTIFACTS_DIR || "/artifacts"
const bootTimeout = Number(process.env.DEMO_BOOT_TIMEOUT_MS || "900000")
const agentTimeout = Number(process.env.DEMO_AGENT_TIMEOUT_MS || "600000")
const prompt = process.env.DEMO_PROMPT ||
  "Create /workspace/workbench-demo.txt containing the current UTC time, verify it with a shell command, then briefly tell me what you did."

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
