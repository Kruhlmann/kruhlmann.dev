/// <reference types="cypress" />
import config from "../../config/config.json";

describe("Theme modal", () => {
    beforeEach(() => {
        cy.visit("/");
        cy.clearCookies();
    });

    it("allow the theme modal to be toggled", () => {
        cy.get("nav > span:nth-child(2)").click();
        cy.get(".modal.theme-modal").should("exist");
        cy.get(".modal.theme-modal .close-btn").click();
        cy.get(".modal.theme-modal").should("not.exist");
        cy.get("nav > span:nth-child(2)").click();
        cy.get(".modal.theme-modal").should("exist");
        cy.get(".modal").click();
        cy.get(".modal.theme-modal").should("not.exist");
    });

    it("allows for theme selection", () => {
        for (const theme of config.themes) {
            cy.get("nav > span:nth-child(2)").click();
            cy.contains(theme.replace(/-/g, " ")).next().click();
            cy.getCookie("theme").should("have.property", "value", theme);
        }
    });
});
