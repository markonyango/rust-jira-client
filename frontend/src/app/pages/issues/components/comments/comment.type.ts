export type Comment = {
  "self": string,
  "id": string,
  "author": {
    "self": string,
    "name": string,
    "key": string,
    "emailAddress": string,
    "avatarUrls": {
      "48x48": string,
      "24x24": string,
      "16x16": string,
      "32x32": string,
    },
    "displayName": string,
    "active": true,
    "timeZone": string
  },
  "body": string,
  "updateAuthor": {
    "self": string,
    "name": string,
    "key": string,
    "emailAddress": string,
    "avatarUrls": {
      "48x48": string,
      "24x24": string,
      "16x16": string,
      "32x32": string,
    },
    "displayName": string,
    "active": true,
    "timeZone": string
  },
  "created": string,
  "updated": string
}

export type CommentWrapper = {
  comments: Comment[],
  maxResults: number,
  total: number,
  startAt: number
}
