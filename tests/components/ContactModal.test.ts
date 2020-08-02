/**
 * @jest-environment jsdom
 */

import * as svelte from "@testing-library/svelte";
import ContactModal from "../../src/components/ContactModal.svelte";
import "@testing-library/jest-dom/extend-expect";

describe(ContactModal.name, () => {
    it("should render", () => {
        const dom = svelte.render(ContactModal);

        const name_header = dom.getByText("andreas s. krühlmann");
        expect(name_header).toBeVisible();

        console.log(JSON.stringify(dom), null, 4);
    });
});
