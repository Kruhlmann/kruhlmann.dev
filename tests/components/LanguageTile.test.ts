/**
 * @jest-environment jsdom
 */

import "@testing-library/jest-dom/extend-expect";

import * as svelte from "@testing-library/svelte";

import LanguageTile from "../../src/components/LanguageTile.svelte";

describe(LanguageTile.name, () => {
    afterEach(svelte.cleanup);

    it("should render", () => {
        const dom = svelte.render(LanguageTile, {
            props: {
                abbreviation: "JS",
                color: "#67ae2a",
                href: "https://example.org",
                usage: 84,
            },
        });

        const anchor = dom.container.querySelector("a");
        expect(anchor).toBeVisible();
        expect(anchor?.getAttribute("href")).toBe("https://example.org");

        const background_color = (anchor as HTMLElement).style.backgroundColor;
        expect(background_color).toBe("rgb(103, 174, 42)");

        const abbreviation_span = dom.getByText("JS");
        expect(abbreviation_span).toBeVisible();

        const usage_span = dom.getByText("84%");
        expect(usage_span).toBeVisible();
    });
});
