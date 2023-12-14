const cheerio = require('cheerio');
const dotenv = require('dotenv');
const puppeteer = require('puppeteer');

dotenv.config();

const username = process.env.USERNAME;
const password = process.env.PASSWORD;

async function schema_mau_se_Login() {
	const login = await fetch('https://schema.mau.se/login_do.jsp', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/x-www-form-urlencoded',
		},
		body: new URLSearchParams({
			username: username,
			password: password,
		}),
	});
	const loginText = await login.text();
	// Using cheerio to parse HTML
	const $ = cheerio.load(loginText);
	const loginFailedMessage = $('.title').text();
	console.log(loginFailedMessage);
}

async function mau_instructure_com_Login() {
	const browser = await puppeteer.launch({ headless: 'new' });
	const page = await browser.newPage();

	await page.goto('https://mau.se/canvas/', { waitUntil: 'networkidle2' });

	// Click the login button
	await page.click('a.button[href="https://mau.instructure.com/login/saml"]');

	// Wait for navigation to the login page
	await page.waitForNavigation({ waitUntil: 'networkidle2' });

	// Fill in the username and password
	await page.type('#userNameInput', username);
	await page.type('#passwordInput', password);

	// Click the submit button
	await page.click('#submitButton');

	try {
		const raceResult = await Promise.race([
			page
				.waitForSelector('#error', { visible: true, timeout: 10000 })
				.then(() => 'error'),
			page
				.waitForNavigation({ waitUntil: 'networkidle0', timeout: 10000 })
				.then(() => 'navigation'),
		]);

		if (raceResult === 'error') {
			const errorMessage = await page.$eval(
				'#errorText',
				(el) => el.textContent
			);
			console.log('Error message:', errorMessage);
		} else if (raceResult === 'navigation') {
			const currentUrl = page.url();
			console.log('Current URL:', currentUrl);

			if (currentUrl.includes('https://mau.instructure.com/?login_success=1')) {
				console.log('Login successful, redirected to the specified URL.');
			}
		}
	} catch (error) {
		console.error(
			'An error occurred or no change within the timeout period:',
			error.message
		);
	}

	await browser.close();
}

//schema_mau_se_Login();

//mau_instructure_com_Login();
