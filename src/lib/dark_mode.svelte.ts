// TODO fix this

export const isDark = $state(
  JSON.parse(localStorage.getItem("isdark") ?? "false")
);
