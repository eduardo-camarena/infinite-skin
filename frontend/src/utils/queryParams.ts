export function generateUrlWithQueryParams(
  baseNewUrl: string,
  searchParams: Record<string, string | number>,
): string {
  const urlSearchParams = new URLSearchParams();
  for (const [key, value] of Object.entries(searchParams)) {
    urlSearchParams.append(key, `${value}`);
  }

  return `${baseNewUrl}?${urlSearchParams.toString()}`;
}
