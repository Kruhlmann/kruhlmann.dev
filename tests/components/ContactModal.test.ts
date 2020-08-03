/**
 * @jest-environment jsdom
 */

import * as svelte from "@testing-library/svelte";
import ContactModal from "../../src/components/ContactModal.svelte";
import "@testing-library/jest-dom/extend-expect";
import { query_select } from "../utils";

describe(ContactModal.name, () => {
    afterEach(svelte.cleanup);

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

        const avatar = query_select(container, "img[alt='avatar']");
        expect(avatar.getAttribute("src")).toBe("/avatar.png");
    });
});
