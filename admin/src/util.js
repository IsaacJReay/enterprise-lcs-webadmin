import React from "react";

const parseJwt = () => {
  try {
    const user = localStorage.getItem("token");
    return user;
  } catch (e) {
    return null;
  }
};

const Utils = (props) => {
  props.history.listen(() => {
    const user = localStorage.getItem("token");
    if (user) {
      const decodedJwt = parseJwt(user);
      if (decodedJwt.exp * 1000 < Date.now()) {
        props.logOut();
      }
    }
  });
  return <div></div>;
};

export default Utils;
