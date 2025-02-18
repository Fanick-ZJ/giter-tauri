export const upToDataElement = (div: HTMLElement, attr: string): HTMLDivElement | undefined => {
    // 向上查找父元素，直到找到 有attr的元素
    let currentElement: HTMLElement | null = div;
    while (currentElement) {
      if (currentElement.hasAttribute(attr)) {
        return currentElement as HTMLDivElement;
      }
      currentElement = currentElement.parentElement;
    }
    return undefined;
  }