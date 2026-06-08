// Real-app smoke test. Launches the actual built desktop binary and clicks
// real buttons, so every step crosses the real window.__TAURI__ IPC bridge.
//
// This is the regression guard for the "buttons do nothing" class of bug:
// if the frontend cannot reach the backend (e.g. withGlobalTauri off, a broken
// command signature, a missing capability), these assertions fail.

describe("OpenPrompter — real IPC smoke", () => {
  it("loads the library shell (frontend mounted)", async () => {
    const heading = await $("h1*=Script Library");
    await heading.waitForDisplayed({ timeout: 30000 });
    await expect(heading).toBeDisplayed();
  });

  it("creates a script via '+ New Script' (proves invoke create_script works)", async () => {
    const newBtn = await $("button*=New Script");
    await newBtn.waitForClickable({ timeout: 20000 });
    await newBtn.click();

    // create_script succeeded -> view switches to the editor and the title
    // input is pre-filled with "New Script". If IPC were broken, the click
    // would silently no-op and this input would never appear.
    const titleInput = await $('input[placeholder="Script title"]');
    await titleInput.waitForDisplayed({ timeout: 20000 });
    await expect(titleInput).toHaveValue("New Script");
  });

  it("edits + auto-saves, then sees it in the library (proves update + list)", async () => {
    const titleInput = await $('input[placeholder="Script title"]');
    await titleInput.setValue("E2E Smoke Script");

    const content = await $('textarea[placeholder="Script content..."]');
    await content.setValue("Hello from the end-to-end smoke test.");

    // Ctrl+S triggers the real update_script command.
    await browser.keys(["Control", "s"]);

    // Back to the library via the sidebar.
    const libNav = await $('[title="Script Library"]');
    await libNav.click();

    // list_scripts must now return the renamed script.
    const item = await $("*=E2E Smoke Script");
    await item.waitForDisplayed({ timeout: 20000 });
    await expect(item).toBeDisplayed();
  });

  it("opens settings without error (proves get_settings works)", async () => {
    const settingsNav = await $('[title="Settings"]');
    await settingsNav.click();
    // Settings panel renders only after get_settings resolves.
    const heading = await $("h1*=Settings");
    await heading.waitForDisplayed({ timeout: 20000 });
    await expect(heading).toBeDisplayed();
  });
});
