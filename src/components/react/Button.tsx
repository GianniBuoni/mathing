import React from "react";
import type { ButtonHTMLAttributes } from "react";

export const buttonColors = {
  primary: "btn-primary",
  secondary: "btn-secondary",
  neutral: "btn-neutral",
  accent: "btn-accent",
};

const hoverColors = {
  neutral: "hover:btn-neutral",
  primary: "hover:btn-primary",
  secondary: "hover:btn-secondary",
  accent: "hover:btn-accent",
};

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
  color?: keyof typeof buttonColors;
  hover?: keyof typeof hoverColors;
  classes?: string;
}

const Button = (
  { color = "neutral", hover = "primary", classes, children, ...rest }: Props,
  ref: React.LegacyRef<HTMLButtonElement>,
) => {
  return (
    <button
      className={`btn btn-sm rounded ${buttonColors[color]} ${hoverColors[hover]} ${classes}`}
      ref={ref}
      {...rest}
    >
      {children}
    </button>
  );
};

export default React.forwardRef(Button);
