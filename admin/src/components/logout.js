import Cookies from "js-cookie";

const Logout = () => {
  // localStorage.removeItem("token");
  Cookies.remove("token");
  window.location.replace("/");
};

export default Logout;
