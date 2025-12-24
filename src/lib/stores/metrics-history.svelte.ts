/**
 * Historical metrics store for time-series data
 * Stores CPU, Memory, Network, Temperature, and Disk I/O metrics over time
 */

export interface CPUDataPoint {
	timestamp: number;
	usage: number;
	coreUsages?: number[];
}

export interface MemoryDataPoint {
	timestamp: number;
	usedMemory: number;
	totalMemory: number;
	swapUsed?: number;
	swapTotal?: number;
	cache?: number;
}

export interface NetworkDataPoint {
	timestamp: number;
	upload: number;
	download: number;
}

export interface TemperatureDataPoint {
	timestamp: number;
	cpu: number;
	gpu?: number;
	system: number;
}

export interface DiskIODataPoint {
	timestamp: number;
	readBytes: number;
	writeBytes: number;
	readOps?: number;
	writeOps?: number;
}

interface MetricsHistory {
	cpu: CPUDataPoint[];
	memory: MemoryDataPoint[];
	network: NetworkDataPoint[];
	temperature: TemperatureDataPoint[];
	diskIO: DiskIODataPoint[];
}

// Maximum number of data points to keep (24 hours at 2-second intervals = ~43,200 points)
// We'll limit to 500 points per metric for performance
const MAX_POINTS = 500;
const RETENTION_MS = 24 * 60 * 60 * 1000; // 24 hours

let history = $state<MetricsHistory>({
	cpu: [],
	memory: [],
	network: [],
	temperature: [],
	diskIO: []
});

/**
 * Add a CPU data point
 */
export function addCPUData(data: Omit<CPUDataPoint, 'timestamp'>) {
	const point: CPUDataPoint = {
		...data,
		timestamp: Date.now()
	};

	history.cpu.push(point);
	pruneData('cpu');
}

/**
 * Add a Memory data point
 */
export function addMemoryData(data: Omit<MemoryDataPoint, 'timestamp'>) {
	const point: MemoryDataPoint = {
		...data,
		timestamp: Date.now()
	};

	history.memory.push(point);
	pruneData('memory');
}

/**
 * Add a Network data point
 */
export function addNetworkData(data: Omit<NetworkDataPoint, 'timestamp'>) {
	const point: NetworkDataPoint = {
		...data,
		timestamp: Date.now()
	};

	history.network.push(point);
	pruneData('network');
}

/**
 * Add a Temperature data point
 */
export function addTemperatureData(data: Omit<TemperatureDataPoint, 'timestamp'>) {
	const point: TemperatureDataPoint = {
		...data,
		timestamp: Date.now()
	};

	history.temperature.push(point);
	pruneData('temperature');
}

/**
 * Add a Disk I/O data point
 */
export function addDiskIOData(data: Omit<DiskIODataPoint, 'timestamp'>) {
	const point: DiskIODataPoint = {
		...data,
		timestamp: Date.now()
	};

	history.diskIO.push(point);
	pruneData('diskIO');
}

/**
 * Prune old data points based on retention and max points
 */
function pruneData(metric: keyof MetricsHistory) {
	const now = Date.now();
	const data = history[metric];

	// Remove data older than retention period
	const filtered = data.filter(point => (now - point.timestamp) <= RETENTION_MS);

	// If still too many points, keep only the most recent MAX_POINTS
	if (filtered.length > MAX_POINTS) {
		history[metric] = filtered.slice(-MAX_POINTS);
	} else {
		history[metric] = filtered;
	}
}

/**
 * Get CPU history filtered by time range
 */
export function getCPUHistory(range: '1h' | '6h' | '24h' | 'all' = 'all'): CPUDataPoint[] {
	return filterByRange(history.cpu, range);
}

/**
 * Get Memory history filtered by time range
 */
export function getMemoryHistory(range: '1h' | '6h' | '24h' | 'all' = 'all'): MemoryDataPoint[] {
	return filterByRange(history.memory, range);
}

/**
 * Get Network history filtered by time range
 */
export function getNetworkHistory(range: '1h' | '6h' | '24h' | 'all' = 'all'): NetworkDataPoint[] {
	return filterByRange(history.network, range);
}

/**
 * Get Temperature history filtered by time range
 */
export function getTemperatureHistory(range: '1h' | '6h' | '24h' | 'all' = 'all'): TemperatureDataPoint[] {
	return filterByRange(history.temperature, range);
}

/**
 * Get Disk I/O history filtered by time range
 */
export function getDiskIOHistory(range: '1h' | '6h' | '24h' | 'all' = 'all'): DiskIODataPoint[] {
	return filterByRange(history.diskIO, range);
}

/**
 * Filter data by time range
 */
function filterByRange<T extends { timestamp: number }>(
	data: T[],
	range: '1h' | '6h' | '24h' | 'all'
): T[] {
	if (range === 'all') return data;

	const now = Date.now();
	const ranges = {
		'1h': 60 * 60 * 1000,
		'6h': 6 * 60 * 60 * 1000,
		'24h': 24 * 60 * 60 * 1000,
	};

	const cutoff = now - ranges[range];
	return data.filter(item => item.timestamp >= cutoff);
}

/**
 * Clear all historical data
 */
export function clearHistory() {
	history = {
		cpu: [],
		memory: [],
		network: [],
		temperature: [],
		diskIO: []
	};
}

/**
 * Get the latest data point for each metric
 */
export function getLatestData() {
	return {
		cpu: history.cpu[history.cpu.length - 1] || null,
		memory: history.memory[history.memory.length - 1] || null,
		network: history.network[history.network.length - 1] || null,
		temperature: history.temperature[history.temperature.length - 1] || null,
		diskIO: history.diskIO[history.diskIO.length - 1] || null
	};
}

/**
 * Reactive getter for CPU history
 */
export function getCPUHistoryReactive(range: '1h' | '6h' | '24h' | 'all' = 'all') {
	return $derived(getCPUHistory(range));
}

/**
 * Reactive getter for Memory history
 */
export function getMemoryHistoryReactive(range: '1h' | '6h' | '24h' | 'all' = 'all') {
	return $derived(getMemoryHistory(range));
}

/**
 * Reactive getter for Network history
 */
export function getNetworkHistoryReactive(range: '1h' | '6h' | '24h' | 'all' = 'all') {
	return $derived(getNetworkHistory(range));
}

/**
 * Reactive getter for Temperature history
 */
export function getTemperatureHistoryReactive(range: '1h' | '6h' | '24h' | 'all' = 'all') {
	return $derived(getTemperatureHistory(range));
}

/**
 * Reactive getter for Disk I/O history
 */
export function getDiskIOHistoryReactive(range: '1h' | '6h' | '24h' | 'all' = 'all') {
	return $derived(getDiskIOHistory(range));
}
