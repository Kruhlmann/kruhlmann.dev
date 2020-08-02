/**
 * @jest-environment jsdom
 */

import * as svelte from "@testing-library/svelte";
import ContactModal from "../../src/components/ContactModal.svelte";
import "@testing-library/jest-dom/extend-expect";

function query_select(container: HTMLElement, query: string): Element {
    const element = container.querySelector(query);
    if (element) {
        return element;
    }
    throw new Error(`Unable to find element matching query ${query}.`);
}

describe(ContactModal.name, () => {
    it("should render", () => {
        const dom = svelte.render(ContactModal);
        const container = dom.container;

        const name_header = dom.getByText("andreas s. krühlmann");
        expect(name_header).toBeVisible();

        const github_anchor = query_select(container, "a[title='GitHub']");
        expect(github_anchor.getAttribute("href")).toBe(
            "https://github.com/kruhlmann",
        );

        const mail_anchor = query_select(container, "a[title='Email']");
        expect(mail_anchor.getAttribute("href")).toBe(
            "mailto:andreas@kruhlmann.dev",
        );

        const linkedin_anchor = query_select(container, "a[title='LinkedIn']");
        expect(linkedin_anchor.getAttribute("href")).toBe(
            "https://linkedin.com/in/andreaskruhlmann",
        );

        const discord_anchor = query_select(container, "a[title='Discord']");
        expect(discord_anchor.getAttribute("href")).toBe(
            "https://discord.gg/Y5eA3dW",
        );

        const gitlab_anchor = query_select(container, "a[title='GitLab']");
        expect(gitlab_anchor.getAttribute("href")).toBe(
            "https://gitlab.com/Kruhlmann",
        );
    });
});