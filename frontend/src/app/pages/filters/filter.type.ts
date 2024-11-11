export type FilterOwner = {
  accountId: string,
    accountType: "atlassian",
    active: boolean,
    avatarUrls: {
      "16x16": string,
      "24x24": string,
      "32x32": string,
      "48x48": string,
    },
    displayName: string,
    key: string,
    name: string,
    self: string
}

export type Filter = {
  approximateLastResult: any,
  description: string,
  favorite: boolean,
  favoriteCount: number,
  id: string,
  jql: string,
  name: string,
  owner: FilterOwner,
  searchUrl: string,
  self: string,
  sharePermissions: any[],
  subscriptions: {
    "end-index": number,
    items: any[],
    "max-results": number,
    size: number,
    "start-index": number,
  },
  viewUrl: string,
}
