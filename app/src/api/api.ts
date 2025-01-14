import { Apity, type ApiResponse } from '@cocreators-ee/apity';
import type { components, paths } from './api.d';

export const apity = Apity.for<paths>();

const forceEnvironment: 'development' | 'production-test' | 'production' | undefined = 'development';

export function getServerUrl(): string {
	const env = forceEnvironment || import.meta.env.VITE_ENV;
	if (env === 'development') {
		return 'http://localhost:8080';
	} else if (env === 'production-test') {
		return 'https://beta.reg.lsv1873.de';
	} else {
		return 'https://reg.lsv1873.de';
	}
}

// global configuration
apity.configure({
    baseUrl: getServerUrl(),
    init: {
      mode: 'cors'
      // headers: {}
    },
  })

export const listOrgs = apity.path('/api/v1/orgs').method('get').create();
export const getOrg = apity.path('/api/v1/org/{org}').method('get').create();
export const updateOrg = apity.path('/api/v1/org/{org}').method('put').create();
export type Org = components["schemas"]["Org"];
export type Event = components["schemas"]["Event"];
