// Proves the teleprompter actually scrolls when you press Play.
//
// Regression guard for the "prompter freezes / nothing happens" bug, where the
// scroll engine advanced ~1 px/s (looked frozen). We create a script, open the
// prompter, press Play, and assert the scrolling text's translateY grows over
// time.

/** Parse the negative translateY (px) out of an element's inline style. */
function translateY(style) {
  if (!style) return 0;
  // matches translate3d(0, -123.4px, 0)
  const m = style.match(/translate3d\(\s*0\s*,\s*-?([0-9.]+)px/);
  return m ? parseFloat(m[1]) : 0;
}

describe("OpenPrompter — prompter scrolls on Play", () => {
  it("creates a script with content", async () => {
    const newBtn = await $("button*=New Script");
    await newBtn.waitForClickable({ timeout: 20000 });
    await newBtn.click();

    const title = await $('input[placeholder="Script title"]');
    await title.waitForDisplayed({ timeout: 20000 });
    await title.setValue("Scroll Test Script");

    const content = await $('textarea[placeholder="Script content..."]');
    // Lots of lines so there is room to scroll.
    await content.setValue(
      Array.from({ length: 60 }, (_, i) => `This is line number ${i + 1}.`).join(
        "\n",
      ),
    );
    await browser.keys(["Control", "s"]);
    await browser.pause(500);

    // Back to library.
    await (await $('[title="Script Library"]')).click();
    const item = await $("*=Scroll Test Script");
    await item.waitForDisplayed({ timeout: 20000 });
  });

  it("opens the prompter and scrolls after Play", async () => {
    // Enter the prompter via the row's Play action.
    const play = await $('[title="Play"]');
    await play.waitForClickable({ timeout: 20000 });
    await play.click();

    const textEl = await $("#prompter-text");
    await textEl.waitForExist({ timeout: 20000 });

    // Press Play in the prompter controls.
    const playBtn = await $("button*=Play");
    await playBtn.waitForClickable({ timeout: 20000 });
    await playBtn.click();

    // Allow for the start countdown (default 3s) then sample movement.
    await browser.pause(5000);
    const y1 = translateY(await textEl.getAttribute("style"));
    await browser.pause(1500);
    const y2 = translateY(await textEl.getAttribute("style"));

    // Must have advanced a visible amount (60 px/s => >> 1px between samples).
    expect(y2).toBeGreaterThan(y1);
    expect(y2).toBeGreaterThan(10);
  });
});
