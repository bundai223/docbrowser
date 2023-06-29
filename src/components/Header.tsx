function Header() {
  return (
    <form className="search">
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
              >
      </input>
    </form>
  )
}

export default Header;
