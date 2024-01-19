/** @type {import('./$types').Actions} */
export const actions = {
    login: async ({ request }) => {
        const data = await request.formData();
        const loginReq: LoginRequest = {
            username: data.get("username") as string,
            pw: data.get("pw") as string
        };

        const res = await fetch("127.0.0.1:8080/api/login", {
            method: "post",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(loginReq)
        });
        const loginRes = await res.json() as LoginResponse;

        if (loginRes.token) {
            localStorage.setItem("admin_auth_token", loginRes.token);
        } else {
            console.log(loginRes.errMsg);
        }
    }
};