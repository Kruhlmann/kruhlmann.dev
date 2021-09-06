/// <reference types="cypress" />
import config from "../../config/config.json";

describe("Theme modal", () => {
    beforeEach(() => {
        cy.visit("/");
        cy.clearCookies();
    });

    it("allow the theme modal to be toggled", () => {
        cy.get("nav").contains("Select theme").click();
        cy.get(".modal.theme-modal").should("exist");
        cy.get(".modal.theme-modal .close-btn").click();
        cy.get(".modal.theme-modal").should("not.exist");
    });

    it("allows for theme selection", () => {
        for (const theme of config.themes) {
            cy.get("nav").contains("Select theme").click();
            cy.contains(theme.replace(/-/g, " ")).next().click();
            cy.getCookie("theme").should("have.property", "value", theme);
        }
    });
});

describe("Contact modal", () => {
    beforeEach(() => {
        cy.visit("/");
        cy.clearCookies();
    });

    it("allow the contact modal to be toggled", () => {
        cy.get("nav").contains("Contact").click();
        cy.get(".modal.contact-modal").should("exist");
        cy.get(".modal.contact-modal .close-btn").click();
        cy.get(".modal.contact-modal").should("not.exist");

        cy.get("main .btn-grp").contains("Contact").click();
        cy.get(".modal.contact-modal").should("exist");
    });

    it("contains the correct links", () => {
        cy.get("nav").contains("Contact").click();
        cy.get("a[href='https://github.com/kruhlmann']").should("exist");
        cy.get("a[href='mailto:andreas@kruhlmann.dev']").should("exist");
        cy.get("a[href='https://linkedin.com/in/andreaskruhlmann']").should("exist");
        cy.get("a[href='https://discord.gg/Y5eA3dW']").should("exist");
        cy.get("a[href='https://gitlab.com/Kruhlmann']").should("exist");
    });
});
