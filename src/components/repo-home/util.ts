export const upToRepoItem = (div: HTMLElement): HTMLDivElement | undefined => {
  // 向上查找父元素，直到找到 有data-repo的元素
  let currentElement: HTMLElement | null = div;
  while (currentElement) {
    if (currentElement.hasAttribute('data-repo')) {
      return currentElement as HTMLDivElement;
    }
    currentElement = currentElement.parentElement;
  }
  return undefined;
}