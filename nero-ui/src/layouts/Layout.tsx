import { type ParentProps } from "solid-js";

export default function Layout(props: ParentProps) {
  return <div class="fixed inset-0">{props.children}</div>;
}
