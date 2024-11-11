export interface JqlResponse {
  expand: string
  startAt: number
  maxResults: number
  total: number
  issues: Issue[]
  warningMessages: any
  names: any
  schema: any
}

export interface Issue {
  expand: string
  id: string
  self: string
  key: string
  fields: Fields
  renderedFields: any
}

export interface Fields {
  issuetype: Issuetype
  components: any[]
  timespent: any
  timeoriginalestimate: any
  timetracking: any;
  description: any
  project: Project
  fixVersions: any[]
  aggregatetimespent: any
  resolution: any
  aggregatetimeestimate: any
  resolutiondate: any
  workratio: number
  summary: string
  lastViewed: string
  watches: Watches
  creator: Creator
  subtasks: any[]
  created: string
  reporter: Reporter
  aggregateprogress: Aggregateprogress
  priority: Priority
  labels: any[]
  environment: any
  timeestimate: any
  aggregatetimeoriginalestimate: any
  versions: any[]
  duedate: any
  progress: Progress
  issuelinks: any[]
  votes: Votes
  assignee: any
  updated: string
  status: Status
}

export interface Issuetype {
  self: string
  id: string
  description: string
  iconUrl: string
  name: string
  subtask: boolean
}

export interface Project {
  self: string
  id: string
  key: string
  name: string
  projectTypeKey: string
  avatarUrls: AvatarUrls
}

export interface AvatarUrls {
  "48x48": string
  "24x24": string
  "16x16": string
  "32x32": string
}

export interface Watches {
  self: string
  watchCount: number
  isWatching: boolean
}

export interface Creator {
  self: string
  name: string
  key: string
  emailAddress: string
  avatarUrls: AvatarUrls2
  displayName: string
  active: boolean
  timeZone: string
}

export interface AvatarUrls2 {
  "48x48": string
  "24x24": string
  "16x16": string
  "32x32": string
}

export interface Reporter {
  self: string
  name: string
  key: string
  emailAddress: string
  avatarUrls: AvatarUrls3
  displayName: string
  active: boolean
  timeZone: string
}

export interface AvatarUrls3 {
  "48x48": string
  "24x24": string
  "16x16": string
  "32x32": string
}

export interface Aggregateprogress {
  progress: number
  total: number
}

export interface Priority {
  self: string
  iconUrl: string
  name: string
  id: string
}

export interface Progress {
  progress: number
  total: number
}

export interface Votes {
  self: string
  votes: number
  hasVoted: boolean
}

export interface Status {
  self: string
  description: string
  iconUrl: string
  name: string
  id: string
  statusCategory: StatusCategory
}

export interface StatusCategory {
  self: string
  id: number
  key: string
  colorName: string
  name: string
}
