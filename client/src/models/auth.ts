type LoginRequest = {
    username: string,
    pw: string
}

type TokenResponse = {
    id: number,
    content: string,
    expires: number
}

type LoginResponse = {
    token?: TokenResponse,
    err?: string
}

type TokenValidationRequest = {
    id: number,
    client: string
}

type TokenValidationReponse = {
    validated: boolean,
    err?: string
}