import messageStore from '@/stores/message';
import uiStore from '@/stores/ui';
import type { PathData, WayNode } from '@/types/api';
import type { BusinessData } from '@/types/business';
import type { LatLngExpression } from 'leaflet';

const BASE_URL = import.meta.env.VITE_API_URL || '';

type Methods = 'GET' | 'PUT' | 'POST' | 'DELETE';

const request = async <T = object, R = object>(
	path: string,
	method: Methods = 'GET',
	data?: T,
	headers = {}
): Promise<R | null> => {
	try {
		uiStore.update((data) => {
			data.isLoading = true;
			return data;
		});
		const res = await fetch(`${BASE_URL}${path}`, {
			method,
			body: JSON.stringify(data),
			headers: {
				'Content-Type': 'application/json',
				...headers
			}
		});
		const contentType = res.headers.get('Content-Type');

		if (res.status === 500 || !contentType?.includes('application/json')) {
			messageStore.update(() => 'Something went wrong!');
			return null;
		}
		const json = await res.json();
		if (!res.ok) {
			messageStore.update(() => {
				return json.message || json.error || 'Failed to fetch a request';
			});
			return null;
		}
		return json;
	} catch (error: unknown) {
		messageStore.update(() => (error as Error).message);
		return null;
	} finally {
		uiStore.update((data) => {
			data.isLoading = false;
			return data;
		});
	}
};

export const searchBusinesses = async (pos: LatLngExpression, rad = 2000) => {
	if (!Array.isArray(pos)) return;
	const [lat, lon] = pos;
	const res = await request<null, { businesses: BusinessData[] }>(
		`/search?lon=${lon}&lat=${lat}&radius=${rad}`
	);
	return res?.businesses;
};

export const getById = async (id: number) => {
	const res = await request<null, { data: BusinessData }>(`/api/businesses/${id}`);
	return res?.data;
};

export const updateBusiness = async (id: number, data: Partial<BusinessData>) => {
	const res = await request<Partial<BusinessData>, { message: string }>(
		`/api/businesses/${id}`,
		'PUT',
		data
	);
	return res?.message;
};

export const getPath = async (data: PathData) => {
	const res = await request<PathData, { path: WayNode[] }>(`/api/createPath`, 'POST', data);
	return res?.path;
};

export const createBusiness = async (data: Omit<BusinessData, 'id'>) => {
	const res = await request<Omit<BusinessData, 'id'>, { id: number }>(
		`/api/businesses`,
		'POST',
		data
	);
	return res?.id;
};
