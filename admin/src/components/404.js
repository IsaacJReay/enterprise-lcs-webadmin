import React from "react";
import { Result, Button } from "antd";
import { Link } from "react-router-dom";
import Cookies from "js-cookie";

const PageNotFound = () => {
  // const isLogin = localStorage.getItem("token");
  const isLogin = Cookies.get("token");
  return (
    <>
      <center>
        <Result
          status="404"
          title="404"
          subTitle="Sorry, the page you visited does not exist."
          extra={
            isLogin ? (
              <>
                <Button type="primary" size="large" className="button-update">
                  <Link to="/status">Back Home</Link>
                </Button>
              </>
            ) : (
              <>
                <Button type="primary" size="large" className="button-update">
                  <Link to="/">Back Home</Link>
                </Button>
              </>
            )
          }
        />
      </center>
    </>
  );
};

export default PageNotFound;
