const puppeteer = require('puppeteer');
const fs = require('fs');
const path = require('path');

(async () => {
    // Ensure docs/images exists
    const docsDir = path.resolve(__dirname, '../../docs/images');
    if (!fs.existsSync(docsDir)){
        fs.mkdirSync(docsDir, { recursive: true });
    }

    console.log('Launching browser...');
    const browser = await puppeteer.launch({ headless: "new" });
    const page = await browser.newPage();
    
    // Set viewport to a nice desktop size
    await page.setViewport({ width: 1280, height: 800 });

    try {
        console.log('Go to Login...');
        await page.goto('http://localhost:3000/login.html');
        
        console.log('Typing credentials...');
        await page.type('#email', 'demo@cluelyguard.com');
        await page.type('#password', 'demo123456');
        
        console.log('Logging in...');
        await Promise.all([
            page.click('button[type="submit"]'),
            page.waitForNavigation({ waitUntil: 'networkidle0' }),
        ]);

        console.log('Capture Dashboard...');
        // Wait for live detections to "load" (fetch)
        await new Promise(r => setTimeout(r, 2000));
        await page.screenshot({ path: path.join(docsDir, 'dashboard.png') });

        console.log('Go to Agents...');
        await page.goto('http://localhost:3000/agents.html');
        await new Promise(r => setTimeout(r, 1000));
        await page.screenshot({ path: path.join(docsDir, 'agents.png') });

        console.log('Go to Alerts...');
        await page.goto('http://localhost:3000/alerts.html');
        await new Promise(r => setTimeout(r, 1000));
        await page.screenshot({ path: path.join(docsDir, 'alerts.png') });

        console.log('Screenshots saved to docs/images/');
    } catch (e) {
        console.error('Error:', e);
    } finally {
        await browser.close();
    }
})();
