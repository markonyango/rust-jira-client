import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable } from 'rxjs';

@Injectable({ providedIn: 'root' })
export class IssuesApiService {

  private http = inject(HttpClient);

  public search(query: string): Observable<JqlResponse> {
    return this.http.get<any>('http://localhost:8080/rest/api/2/search', {
      params: {
        jql: query
      },
      headers: {
        Authorization: `Basic ${btoa('admin'+':'+'admin')}`
      }
    })
  }

  public fetchAll(): Observable<any> {
    const bodyData = `{
      "expand": [
        "names"
      ],
      "fields": [
        "summary",
        "project",
        "assignee"
      ],
      "fieldsByKeys": false,
      "issueIdsOrKeys": [
        "OPD-1",
        "OPD-2",
        "10003"
      ],
      "properties": []
    }`;
    return this.http.post('http://localhost:8080/rest/api/2/issue/bulkfetch', bodyData, {
      headers: {
        Authorization: `Basic ${btoa('admin'+':'+'admin')}`,
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      }
    })
  }

  public getIssue(id: string) {
    return this.http.get<any>(`http://localhost:8080/rest/api/2/issue/${id}`, {
      headers: {
        Authorization: `Basic ${btoa('admin'+':'+'admin')}`
      }
    })
  }
}

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
  customfield_10110: any
  customfield_10111: any
  aggregatetimespent: any
  resolution: any
  customfield_10104: any
  customfield_10105: string
  customfield_10106: any
  customfield_10107: any
  customfield_10108: any
  aggregatetimeestimate: any
  customfield_10109: any
  resolutiondate: any
  workratio: number
  summary: string
  lastViewed: string
  watches: Watches
  creator: Creator
  subtasks: any[]
  created: string
  reporter: Reporter
  customfield_10000: string
  aggregateprogress: Aggregateprogress
  priority: Priority
  customfield_10100: any
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

const JQL_MOCK_RESPONSE = {
  "expand": "names,schema",
  "startAt": 0,
  "maxResults": 50,
  "total": 1,
  "issues": [
    {
      "expand": "operations,versionedRepresentations,editmeta,changelog,renderedFields",
      "id": "10000",
      "self": "http://localhost:8080/rest/api/2/issue/10000",
      "key": "OPD-1",
      "fields": {
        "issuetype": {
          "self": "http://localhost:8080/rest/api/2/issuetype/10001",
          "id": "10001",
          "description": "",
          "iconUrl": "http://localhost:8080/images/icons/issuetypes/story.svg",
          "name": "Story",
          "subtask": false
        },
        "components": [],
        "timespent": null,
        "timeoriginalestimate": null,
        "description": null,
        "project": {
          "self": "http://localhost:8080/rest/api/2/project/10000",
          "id": "10000",
          "key": "OPD",
          "name": "OPD",
          "projectTypeKey": "software",
          "avatarUrls": {
            "48x48": "http://localhost:8080/secure/projectavatar?avatarId=10324",
            "24x24": "http://localhost:8080/secure/projectavatar?size=small&avatarId=10324",
            "16x16": "http://localhost:8080/secure/projectavatar?size=xsmall&avatarId=10324",
            "32x32": "http://localhost:8080/secure/projectavatar?size=medium&avatarId=10324"
          }
        },
        "fixVersions": [],
        "customfield_10110": null,
        "customfield_10111": null,
        "aggregatetimespent": null,
        "resolution": null,
        "customfield_10104": null,
        "customfield_10105": "0|hzzzzz:",
        "customfield_10106": null,
        "customfield_10107": null,
        "customfield_10108": null,
        "aggregatetimeestimate": null,
        "customfield_10109": null,
        "resolutiondate": null,
        "workratio": -1,
        "summary": "Test Vorgang",
        "lastViewed": "2024-10-30T21:39:39.357+0000",
        "watches": {
          "self": "http://localhost:8080/rest/api/2/issue/OPD-1/watchers",
          "watchCount": 1,
          "isWatching": true
        },
        "creator": {
          "self": "http://localhost:8080/rest/api/2/user?username=admin",
          "name": "admin",
          "key": "JIRAUSER10000",
          "emailAddress": "adeptus.noobus@gmail.com",
          "avatarUrls": {
            "48x48": "https://www.gravatar.com/avatar/85b630d2aa15d6ef942ab7165bd8d4ca?d=mm&s=48",
            "24x24": "https://www.gravatar.com/avatar/85b630d2aa15d6ef942ab7165bd8d4ca?d=mm&s=24",
            "16x16": "https://www.gravatar.com/avatar/85b630d2aa15d6ef942ab7165bd8d4ca?d=mm&s=16",
            "32x32": "https://www.gravatar.com/avatar/85b630d2aa15d6ef942ab7165bd8d4ca?d=mm&s=32"
          },
          "displayName": "Adeptus Noobus",
          "active": true,
          "timeZone": "Etc/UTC"
        },
        "subtasks": [],
        "created": "2024-10-27T21:43:23.000+0000",
        "reporter": {
          "self": "http://localhost:8080/rest/api/2/user?username=admin",
          "name": "admin",
          "key": "JIRAUSER10000",
          "emailAddress": "adeptus.noobus@gmail.com",
          "avatarUrls": {
            "48x48": "https://www.gravatar.com/avatar/85b630d2aa15d6ef942ab7165bd8d4ca?d=mm&s=48",
            "24x24": "https://www.gravatar.com/avatar/85b630d2aa15d6ef942ab7165bd8d4ca?d=mm&s=24",
            "16x16": "https://www.gravatar.com/avatar/85b630d2aa15d6ef942ab7165bd8d4ca?d=mm&s=16",
            "32x32": "https://www.gravatar.com/avatar/85b630d2aa15d6ef942ab7165bd8d4ca?d=mm&s=32"
          },
          "displayName": "Adeptus Noobus",
          "active": true,
          "timeZone": "Etc/UTC"
        },
        "customfield_10000": "{summaryBean=com.atlassian.jira.plugin.devstatus.rest.SummaryBean@cf37b4[summary={pullrequest=com.atlassian.jira.plugin.devstatus.rest.SummaryItemBean@301b1730[byInstanceType={},overall=PullRequestOverallBean{stateCount=0, state='OPEN', details=PullRequestOverallDetails{openCount=0, mergedCount=0, declinedCount=0}}], build=com.atlassian.jira.plugin.devstatus.rest.SummaryItemBean@64f5a50[byInstanceType={},overall=com.atlassian.jira.plugin.devstatus.summary.beans.BuildOverallBean@6e057f3e[failedBuildCount=0,successfulBuildCount=0,unknownBuildCount=0,count=0,lastUpdated=<null>,lastUpdatedTimestamp=<null>]], review=com.atlassian.jira.plugin.devstatus.rest.SummaryItemBean@21c50316[byInstanceType={},overall=com.atlassian.jira.plugin.devstatus.summary.beans.ReviewsOverallBean@7bdfbf4d[dueDate=<null>,overDue=false,state=<null>,stateCount=0,count=0,lastUpdated=<null>,lastUpdatedTimestamp=<null>]], deployment-environment=com.atlassian.jira.plugin.devstatus.rest.SummaryItemBean@7bee7bc[byInstanceType={},overall=com.atlassian.jira.plugin.devstatus.summary.beans.DeploymentOverallBean@420794fa[showProjects=false,successfulCount=0,topEnvironments=[],count=0,lastUpdated=<null>,lastUpdatedTimestamp=<null>]], repository=com.atlassian.jira.plugin.devstatus.rest.SummaryItemBean@4b53f21f[byInstanceType={},overall=com.atlassian.jira.plugin.devstatus.summary.beans.CommitOverallBean@4d7019ef[count=0,lastUpdated=<null>,lastUpdatedTimestamp=<null>]], branch=com.atlassian.jira.plugin.devstatus.rest.SummaryItemBean@3d0384fa[byInstanceType={},overall=com.atlassian.jira.plugin.devstatus.summary.beans.BranchOverallBean@43dd67e4[count=0,lastUpdated=<null>,lastUpdatedTimestamp=<null>]]},configErrors=[],errors=[]], devSummaryJson={\"cachedValue\":{\"errors\":[],\"configErrors\":[],\"summary\":{\"pullrequest\":{\"overall\":{\"count\":0,\"lastUpdated\":null,\"stateCount\":0,\"state\":\"OPEN\",\"details\":{\"openCount\":0,\"mergedCount\":0,\"declinedCount\":0,\"total\":0},\"open\":true},\"byInstanceType\":{}},\"build\":{\"overall\":{\"count\":0,\"lastUpdated\":null,\"failedBuildCount\":0,\"successfulBuildCount\":0,\"unknownBuildCount\":0},\"byInstanceType\":{}},\"review\":{\"overall\":{\"count\":0,\"lastUpdated\":null,\"stateCount\":0,\"state\":null,\"dueDate\":null,\"overDue\":false,\"completed\":false},\"byInstanceType\":{}},\"deployment-environment\":{\"overall\":{\"count\":0,\"lastUpdated\":null,\"topEnvironments\":[],\"showProjects\":false,\"successfulCount\":0},\"byInstanceType\":{}},\"repository\":{\"overall\":{\"count\":0,\"lastUpdated\":null},\"byInstanceType\":{}},\"branch\":{\"overall\":{\"count\":0,\"lastUpdated\":null},\"byInstanceType\":{}}}},\"isStale\":false}}",
        "aggregateprogress": {
          "progress": 0,
          "total": 0
        },
        "priority": {
          "self": "http://localhost:8080/rest/api/2/priority/3",
          "iconUrl": "http://localhost:8080/images/icons/priorities/medium.svg",
          "name": "Medium",
          "id": "3"
        },
        "customfield_10100": null,
        "labels": [],
        "environment": null,
        "timeestimate": null,
        "aggregatetimeoriginalestimate": null,
        "versions": [],
        "duedate": null,
        "progress": {
          "progress": 0,
          "total": 0
        },
        "issuelinks": [],
        "votes": {
          "self": "http://localhost:8080/rest/api/2/issue/OPD-1/votes",
          "votes": 0,
          "hasVoted": false
        },
        "assignee": null,
        "updated": "2024-10-27T21:43:23.000+0000",
        "status": {
          "self": "http://localhost:8080/rest/api/2/status/10000",
          "description": "",
          "iconUrl": "http://localhost:8080/",
          "name": "Backlog",
          "id": "10000",
          "statusCategory": {
            "self": "http://localhost:8080/rest/api/2/statuscategory/2",
            "id": 2,
            "key": "new",
            "colorName": "default",
            "name": "Zu erledigen"
          }
        }
      },
      "renderedFields": null
    }
  ],
  "warningMessages": null,
  "names": null,
  "schema": null
}
