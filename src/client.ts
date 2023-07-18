// client.ts

import { Procedures } from '@/types/rspc/bindings' // 自動生成した型定義ファイル
import { createClient } from '@rspc/client'
import { TauriTransport } from '@rspc/tauri'

export const tauriClient = createClient<Procedures>({
	transport: new TauriTransport(),
})