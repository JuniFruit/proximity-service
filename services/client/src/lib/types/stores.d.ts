export type UIStore = {
	businessSelected: number;
	isSimMoving: boolean;
	isChoosingPoint: boolean;
	onPosConfirmed?: (pos: [number, number]) => void;
};

export type ConfigStore = {
	requestDebounceRadius: number;
	requestRadius: number;
	maxVisibleRadius: number;
	simMovementMode: 'fly' | 'drive';
};
