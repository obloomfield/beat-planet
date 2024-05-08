"use client";

import React, { useEffect } from "react";

export const EngineTest = () => {
  return (
    <iframe
      src={"../../lib/engine.js"}
      title="External Script"
      width="800"
      height="600"
    ></iframe>
  );
};
