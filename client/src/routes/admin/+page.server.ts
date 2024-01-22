import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "../login/$types";

export const load: PageServerLoad = async ({ cookies }) => {
    const token = cookies.get("token");

    if (token) {
        const req = handleTokenValidationRequest(token);
        const res = await fetch("http://127.0.0.1:8080/api/validate", {
            method: "post",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(req)
        });

        if (res.ok) {
            const tokenValidRes: TokenValidationReponse = await res.json();
            if (!tokenValidRes?.validated) {
                if (tokenValidRes?.err) {
                    handleInvalidToken(tokenValidRes.err);
                }
            }
        } else {
            handleInvalidToken(await res.text());
        }
    }
};

function handleInvalidToken(err: string) {
    console.error(err);
    redirect(301, "/login");
}

function handleTokenValidationRequest(value: string): TokenValidationRequest {
    let params = value.split(';');

    return {
        id: parseInt(handleParamValue(params.at(0) as string)),
        client: handleParamValue(params.at(1) as string)
    }
}

const handleParamValue = (param: string): string => param.split('=').at(1) as string;