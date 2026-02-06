const puppeteer = require('puppeteer');
const fs = require('fs');
const path = require('path');

(async () => {
    const docsDir = path.resolve(__dirname, '../../docs/images');
    if (!fs.existsSync(docsDir)){
        fs.mkdirSync(docsDir, { recursive: true });
    }

    const outputText = `
C:\\Users\\brand\\code\\Anticheat\\client_sdk> cargo run
   Compiling client_sdk v0.1.0 (C:\\Users\\brand\\code\\Anticheat\\client_sdk)
    Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running \`target\\debug\\client_sdk.exe\`
Starting Game Client Simulation (Agent ID: 2a6e5402-5515-48ec-99f8-2373c8ded8a3)
Connecting to Anticheat Server at http://localhost:3000...
Sending Heartbeat & Scan Results...
--> POST /ingest/batch
{
  "id": "964b4dca-1610-4418-aa0a-40bf63b35f64",
  "agent_id": "2a6e5402-5515-48ec-99f8-2373c8ded8a3",
  "detection_type": "memory_scan",
  "severity": "low",
  "title": "Routine Scan Completed",
  "description": "No anomalies detected in process memory.",
  "metadata": {
    "scanned_regions": 1024
  },
  "timestamp": "2026-02-06T12:45:47.941085200+00:00"
}
Sleeping for 10 seconds...
    `.trim();

    const htmlContent = `
    <html>
    <body style="margin:0; padding:0; background: #1e1e1e;">
        <div style="padding: 20px; font-family: 'Consolas', 'Courier New', monospace; font-size: 14px; color: #d4d4d4; line-height: 1.5;">
            <div style="margin-bottom: 10px; color: #808080;"># Simulated Terminal Output</div>
            <pre style="margin: 0; white-space: pre-wrap;">${outputText}</pre>
        </div>
    </body>
    </html>
    `;

    console.log('Launching browser...');
    const browser = await puppeteer.launch({ headless: "new" });
    const page = await browser.newPage();
    
    await page.setViewport({ width: 800, height: 600 });
    await page.setContent(htmlContent);

    console.log('Capturing Client SDK screenshot...');
    await page.screenshot({ path: path.join(docsDir, 'client_sdk.png') });
    
    console.log('Saved to client_sdk.png');
    await browser.close();
})();
