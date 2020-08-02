/**
 * @jest-environment jsdom
 */

import * as svelte from "@testing-library/svelte";
import LanguageStats from "../../src/components/LanguageStats.svelte";
import "@testing-library/jest-dom/extend-expect";

const user_base_url = "https://github.com/Kruhlmann/?tab=repositories";
const language_records = [
    { language: "JavaScript", link: "/?lang=JavaScript", hits: 15 },
    { language: "Python", link: "/?lang=Python", hits: 11 },
];
const max_hits = Math.max(...language_records.map((record) => record.hits));

function make_github_link(link: string): string {
    return `${user_base_url}&language=${link.slice(7)}`;
}

function class_name(classname: string): string {
    return classname.replace(/[\W_]+/g, "-");
}

describe(LanguageStats.name, () => {
    afterEach(svelte.cleanup);

    it("should render", () => {
        const dom = svelte.render(LanguageStats, {
            props: { language_records },
        });
        const container = dom.container;

        for (const { language, hits, link } of language_records) {
            const href = make_github_link(link);
            const anchor = dom.getByText(language);
            expect(anchor).toBeVisible();
            expect(anchor.getAttribute("href")).toBe(href);

            const hits_query = `.hits-count.${class_name(language)}`;
            const hits_count =
                container.querySelector(hits_query)?.textContent || "";
            expect(hits_count).toBe(`${hits}`);

            const expected_width = (hits / max_hits) * 100;
            const hits_bar = <HTMLElement>(
                container.querySelector(`.hits-bar.${class_name(language)}`)
            );
            console.log(expected_width);
            expect(hits_bar.style.width).toBe(`${expected_width}%`);
        }
    });
});
