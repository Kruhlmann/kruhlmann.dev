import * as sapper from "@sapper/server";
import compression from "compression";
import express from "express";
import sirv from "sirv";

import { business_card_response } from "../lib/business_card";

const { PORT, NODE_ENV, BASEPATH } = process.env;
const development = NODE_ENV === "development";
const curl_user_agent_pattern = /^curl\/\d+\.\d+\.\d+$/;

function show_business_card_middleware(
    request: express.Request,
    response: express.Response,
    next: express.NextFunction,
) {
    const agent = request.headers["user-agent"];
    const agent_is_curl = curl_user_agent_pattern.test(`${agent}`);
    if (agent_is_curl) {
        response.send(business_card_response);
    } else {
        next();
    }
}

const middleware = [
    `/${BASEPATH || ""}`,
    show_business_card_middleware,
    compression({ threshold: 0 }),
    sirv("static", { dev: development }),
    sapper.middleware(),
];

express()
    .use(...middleware)
    .listen(PORT, console.error);
