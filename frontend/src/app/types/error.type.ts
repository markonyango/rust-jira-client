export type HttpNotFoundError = {
  message: string,
  "status-code": 404
}

export type HttpBadRequestError = {
  message: string,
  "status-code": 400
}

export type HttpUnauthorizedError = {
  message: string,
  "status-code": 401
}

export type Error = HttpNotFoundError | HttpBadRequestError | HttpUnauthorizedError;
