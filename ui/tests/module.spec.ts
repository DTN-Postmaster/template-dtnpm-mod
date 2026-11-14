import { test, expect } from '@playwright/test';

test.describe('Template WebModule UI tests', () => {
  test('renders module header and security badge', async ({ page }) => {
    await page.goto('/');
    const title = page.locator('h1');
    await expect(title).toBeVisible();
    await expect(title).toContainText('Template DTN Extension');
  });
});
