import React, { useEffect, useState } from "react";
import { Row, Col, Button, message, Spin } from "antd";
import { SyncOutlined } from "@ant-design/icons";
import axios from "axios";
import Cookies from "js-cookie";

const DynamicUpdate = () => {
  // const getToken = localStorage.getItem("token");
  const getToken = Cookies.get("token");
  const baseUrl = process.env.REACT_APP_API_URL;
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  // =========state ============

  const [updates, setUpdates] = useState([]);
  const [loading, setLoading] = useState(false);

  // ===========get update data =============

  async function fetchData() {
    await axios({
      method: "GET",
      url: `${baseUrl}/settings/update/status`,
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setLoading(true);
        setUpdates(res.data);
        setLoading(false);
      })
      .catch((err) => console.log(err));
  }

  useEffect(() => {
    setInterval(() => {
      fetchData();
    }, 1000);
  }, []);

  const onReload = () => {
    window.location.reload();
  };

  // ========= operation update ==========

  const handleUpdates = (e, id, sys_update) => {
    e.preventDefault();
    const inputData = {
      id: id,
      sys_update: sys_update,
    };

    axios
      .post(`${baseUrl}/settings/update/update`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })

      .then((res) => {
        if ((res.statusCode = 200)) {
          setLoading(true);
          fetchData();
          setLoading(false);
        } else {
          message.error("Operation Failed! ");
        }
      })
      .catch((err) => console.log(err));
  };

  if (loading) {
    return (
      <center>
        <Spin />
      </center>
    );
  }

  return (
    <React.Fragment>
      <div className="sub-header">
        <div className="icons-update" onClick={onReload}>
          <SyncOutlined className="updates" />
          Check For Update
        </div>
      </div>
      <div className="container-update">
        {updates.map((res) => {
          function formatBytes(zise, decimals = 2) {
            if (zise === 0) return "0 B";
            const k = 1024;
            const dm = decimals < 0 ? 0 : decimals;
            const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
            const total = Math.floor(Math.log(zise) / Math.log(k));
            return (
              parseFloat((zise / Math.pow(k, total)).toFixed(dm)) +
              " " +
              sizes[total]
            );
          }

          return (
            <div className="item-storages">
              <Row gutter={[12, 12]}>
                <Col span={19}>
                  <div>
                    <h3>{res.display_name}</h3>
                    <p>Size: {formatBytes(res.update_size)}</p>
                  </div>
                </Col>
                <Col span={5}>
                  {res.status === "Downloading" | res.status === "Installing" && (<Spin/>)}
                  {res.status === "Installing" && (
                    <Button
                      type="primary"
                      className="button-update"
                      disabled
                    >
                      Installing
                    </Button>
                  )}
                  {res.status === "Downloading" && (
                    <Button
                      type="primary"
                      className="button-update"
                      disabled
                    >
                      Downloading
                    </Button>
                  )}
                  {res.status === "New" && (
                    <Button
                      type="primary"
                      className="button-update"
                      onClick={(e) => handleUpdates(e, res.id, res.sys_update)}
                    >
                      Update
                    </Button>
                  )}
                </Col>
              </Row>
            </div>
          );
        })}
      </div>
    </React.Fragment>
  );
};

export default DynamicUpdate;
