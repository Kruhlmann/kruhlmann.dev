export function query_select(container: HTMLElement, query: string): Element {
    if (!container.querySelector) {
        throw new Error(
            `${JSON.stringify(container)} does not allow query selection.`,
        );
    }
    const element = container.querySelector(query);
    if (element) {
        return element;
    }
    throw new Error(`Unable to find element matching query ${query}.`);
}
