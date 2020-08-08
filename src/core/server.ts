import * as sapper from "@sapper/server";
import compression from "compression";
import express from "express";
import sirv from "sirv";

const { PORT, NODE_ENV, BASEPATH } = process.env;
const development = NODE_ENV === "development";
const curl_user_agent_pattern = /^curl\/\d+\.\d+\.\d+$/;
const ansi_yel = "\u001B[33m";
const ansi_rst = "\u001B[0m";
const ansi_und = "\u001B[4m";
const ansi_bld = "\u001B[1m";
const business_card_response = `
${ansi_yel}╔═════════════════════════════════════════════════════════════════════════════╗${ansi_rst}
${ansi_yel}║${ansi_rst}                                                                             ${ansi_yel}║${ansi_rst}
${ansi_yel}║${ansi_rst}                              ${ansi_bld}${ansi_und}Andreas Krühlmann${ansi_rst}                              ${ansi_yel}║${ansi_rst}
${ansi_yel}║${ansi_rst}                                                                             ${ansi_yel}║${ansi_rst}
${ansi_yel}║${ansi_rst}                      Self-taught software developer                         ${ansi_yel}║${ansi_rst}
${ansi_yel}║${ansi_rst}                     working for Siemens Mobility A/S                        ${ansi_yel}║${ansi_rst}
${ansi_yel}║${ansi_rst}                                                                             ${ansi_yel}║${ansi_rst}
${ansi_yel}║${ansi_rst}      ${ansi_bld}${ansi_und}Languages:${ansi_rst}       ${ansi_bld}${ansi_und}Frameworks:${ansi_rst}         ${ansi_bld}${ansi_und}Find me:${ansi_rst}                          ${ansi_yel}║${ansi_rst}
${ansi_yel}║${ansi_rst}      Typescript       SvelteJS            GitHub: Kruhlmann                 ${ansi_yel}║${ansi_rst}
${ansi_yel}║${ansi_rst}      Ruby             Flask               Discord: ges#0001                 ${ansi_yel}║${ansi_rst}
${ansi_yel}║${ansi_rst}      Python           LibGDX              LinkedIn: andreaskruhlmann        ${ansi_yel}║${ansi_rst}
${ansi_yel}║${ansi_rst}      Java             discord.js          Email: andreas@kruhlmann.dev      ${ansi_yel}║${ansi_rst}
${ansi_yel}║${ansi_rst}                                                                             ${ansi_yel}║${ansi_rst}
${ansi_yel}╚═════════════════════════════════════════════════════════════════════════════╝${ansi_rst}
`;

function curl_redirect_middleware(
    request: express.Request,
    response: express.Response,
    next: express.NextFunction,
) {
    const agent = request.headers["user-agent"];
    const agent_is_curl = curl_user_agent_pattern.test(`${agent}`);
    console.log(agent);
    console.log(agent_is_curl);
    if (agent_is_curl) {
        response.send(business_card_response);
    } else {
        next();
    }
}

const middleware = [
    `/${BASEPATH || ""}`,
    curl_redirect_middleware,
    compression({ threshold: 0 }),
    sirv("static", { dev: development }),
    sapper.middleware(),
];

express()
    .use(...middleware)
    .listen(PORT, console.error);
