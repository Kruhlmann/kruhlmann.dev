export function query_select(container: HTMLElement, query: string): Element {
    const element = container.querySelector(query);
    if (element) {
        return element;
    }
    throw new Error(`Unable to find element matching query ${query}.`);
}
