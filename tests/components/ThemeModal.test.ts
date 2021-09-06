/**
 * @jest-environment jsdom
 */

import "@testing-library/jest-dom/extend-expect";

import * as svelte from "@testing-library/svelte";

import config from "../../config/config.json";
import ThemeModal from "../../src/components/ThemeModal.svelte";
import { query_select } from "../utils";

describe(ThemeModal.name, () => {
    afterEach(svelte.cleanup);

    it("should render", () => {
        const dom = svelte.render(ThemeModal);
        const container = dom.container;

        for (const theme of config.themes) {
            const title = query_select(container, `.theme.theme-${theme} span.title`);
            const select_button = query_select(container, `.theme.theme-${theme} span.select-btn`);
            expect(title).toBeVisible();
            expect(select_button).toBeVisible();
        }
    });
});
