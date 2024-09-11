import { writable } from 'svelte/store';
import { getDriveInfo } from "$lib/api";

export type FolderStructure = {
  name: string;
  path: string;
  children: FolderStructure[] | null;
};

export type DriveState = {
  [key: string]: FolderStructure | null;
};

// Drive List 
export let drives = writable<DriveState>({});

// API로부터 드라이브 정보를 가져와서 drives 스토어를 업데이트하는 함수
export async function updateDrives() {
  try {
    const driveInfoArray = await getDriveInfo();
    // console.log(driveInfoArray)
    // 받은 드라이브 정보에서 mount_point를 추출(mount_point는 드라이브명 문자열값에 대응되는 Key)
    const driveState: DriveState = driveInfoArray.reduce((acc: DriveState, drive: any) => {
      acc[drive.mount_point] = null; // 각 드라이브의 mount_point를 키로 사용하고 값을 null로 설정
      return acc;
    }, {});

    drives.set(driveState);
  } catch (error) {
    console.error('Failed to update drives:', error);
  }
}
