type LoginRequest = {
    username: string,
    pw: string
}

type LoginResponse = {
    token?: string
    err?: string
}