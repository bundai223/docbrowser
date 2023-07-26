import { ChangeEvent, FormEvent, useState } from 'react';
import Button from './atom/Button';
import './Header.css'
// import { invoke } from '@tauri-apps/api';
import { tauriClient } from '@/client'
import { SearchResult as RawSearchResult, SearchIndex as RawSearchIndex } from '@/types/rspc/bindings';

const search = (search_word: string): Promise<RawSearchResult> => {
	// return await invoke('get_app_name')
	return tauriClient.query(['app.search', search_word]) // エディタ補完が効く
}

function toConfig() {
  console.log('to config.')
}

type Props = {
  searchHandler?: SearchHandler
}

export type SearchResult = RawSearchResult;
export type SearchIndex = RawSearchIndex;
export type SearchHandler = (searched: SearchResult) => void

function Header(props: Props) {
  // console.log(props)
  const [ search_query, setSearchQuery ] = useState<string>('')

  async function onChange(e: ChangeEvent<HTMLInputElement>) {
    const _query = e.currentTarget.value;
    setSearchQuery(_query);

    const result = await search(_query);

    if (props.searchHandler) {
      props.searchHandler(result);
    }
  }

  function onSubmit(e: FormEvent) {
    e.preventDefault();
  }


  return (
    <div className="header">
      <form className="search" onSubmit={onSubmit}>
        <input type="search"
          name="q"
          className="_search-input"
          placeholder="Search…"
          autoComplete="off"
          autoCapitalize="off"
          autoCorrect="off"
          spellCheck="false"
          maxLength={30}
          aria-label="Search"
          onChange={onChange}
        >
        </input>
      </form>
      <Button onClick={toConfig}>・・・</Button>
    </div>
  )
}

export default Header;
