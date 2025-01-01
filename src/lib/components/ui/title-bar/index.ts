import type { HTMLAttributes } from "svelte/elements";
import Root from "./title-bar.svelte";

type Props = HTMLAttributes<HTMLElement> & {
  class?: string;
};

export {
  Root,
  type Props,
  //
  Root as TitleBar,
  type Props as TitleBarProps,
};
