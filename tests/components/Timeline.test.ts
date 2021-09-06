/**
 * @jest-environment jsdom
 */

import "@testing-library/jest-dom/extend-expect";

import * as svelte from "@testing-library/svelte";

import config from "../../config/config.json";
import Timeline from "../../src/components/Timeline.svelte";
import { TimelineItem } from "../../src/types";

const years_mapped = config.activities.map((activity: TimelineItem) => {
    return activity.date.year;
});
const years = [...new Set(years_mapped), new Date().getFullYear()];

describe(Timeline.name, () => {
    afterEach(svelte.cleanup);

    it("should render", () => {
        const dom = svelte.render(Timeline);

        for (const year of years) {
            const year_node = dom.getByText(`${year}`);
            expect(year_node).toBeVisible();
        }
    });
});
