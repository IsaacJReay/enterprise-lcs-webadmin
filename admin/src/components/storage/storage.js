import React, { useState, useEffect } from "react";
import {
  Layout,
  Col,
  Row,
  Collapse,
  Progress,
  Space,
  message,
  Dropdown,
} from "antd";
import { CaretRightOutlined, NodeCollapseOutlined } from "@ant-design/icons";
import myImage from "../../assets/images/Hard-Drive3.png";
import driver from "../../assets/images/Hard-Drive.png";
import { Link } from "react-router-dom";
import axios from "axios";
import { IoIosHelpCircle } from "react-icons/io";
import Cookies from "js-cookie";

const { Content } = Layout;
const { Panel } = Collapse;

const Storages = () => {
  // -------state management ---------------
  const [storages, setStorages] = useState([]);

  // -------token ----------
  const baseUrl = process.env.REACT_APP_API_URL;
  // const getToken = localStorage.getItem("token");
  const getToken = Cookies.get("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  // -------- get data ----------

  async function fetchData() {
    axios
      .get(`${baseUrl}/settings/storage/status`, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        setStorages(res.data);
      })
      .catch((err) => console.log(err));
  }

  useEffect(() => {
    fetchData();
  }, []);

  // ============handle unmount ============

  const handleUnmount = (value) => {
    const unmountData = {
      drive_partuuid: value,
    };

    axios
      .post(`${baseUrl}/settings/storage/device/unmount`, unmountData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        if ((res.statusCode = 200)) {
          message.success("Successful!");
          window.location.replace("/storages");
        } else {
          message.error("Operation Failed! ");
        }
      })
      .catch((err) => {
        console.log(err);
      });
  };

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card">
              <div className="container">
                <div className="container-header">
                  <h1>File Storages</h1>
                </div>
                <hr />

                <Collapse
                  bordered={false}
                  defaultActiveKey={["1"]}
                  expandIcon={({ isActive }) => (
                    <CaretRightOutlined rotate={isActive ? 90 : 0} />
                  )}
                >
                  <Panel
                    className="storage-header"
                    header={<span>All devices ({storages.length})</span>}
                    key="1"
                  >
                    {storages.map((res, index) => {
                      const options = (
                        <div
                          style={{ width: "170px" }}
                          className="options-right-click"
                        >
                          <Row
                            className="accountNavbarhover"
                            onClick={() =>
                              handleUnmount(res.drive_partuuid.drive_partuuid)
                            }
                          >
                            <Col span={4} style={{ paddingTop: "6px" }}>
                              <NodeCollapseOutlined className="about-us-icon" />
                            </Col>
                            <Col span={20} className="logout">
                              <p style={{ color: "black" }}>Safely Remove</p>
                            </Col>
                          </Row>
                        </div>
                      );

                      if (index === 0) {
                        return (
                          <div className="storages-container">
                            <div className="item-storages">
                              <Link
                                to={`/storages/setting/${res.drive_partuuid.drive_partuuid}`}
                              >
                                <Row gutter={24}>
                                  <Col span={3}>
                                    <img
                                      src={myImage}
                                      className="storages-icon"
                                      alt="storages"
                                    />
                                  </Col>
                                  <Col span={21}>
                                    <p>{res.drive_label}</p>
                                    <Progress
                                      percent={res.percentage}
                                      showInfo={false}
                                      strokeWidth={25}
                                      status="active"
                                      strokeColor={{
                                        "100%": "#1890ff",
                                      }}
                                    />
                                    <p>
                                      {res.free_space} free of {res.total_space}
                                    </p>
                                  </Col>
                                </Row>
                              </Link>
                            </div>
                          </div>
                        );
                      } else {
                        return (
                          <div className="storages-container">
                            <div className="item-storages">
                              <Link
                                to={`/storages/setting/${res.drive_partuuid.drive_partuuid}`}
                              >
                                <Dropdown
                                  overlay={options}
                                  trigger={["contextMenu"]}
                                >
                                  <Row gutter={24}>
                                    <Col span={3}>
                                      <img
                                        src={driver}
                                        className="storages-icon2"
                                        alt="storages "
                                      />
                                    </Col>
                                    <Col span={21}>
                                      <p>{res.drive_label}</p>
                                      <Progress
                                        percent={res.percentage}
                                        showInfo={false}
                                        strokeWidth={25}
                                        status="active"
                                        strokeColor={{
                                          "100%": "#1890ff",
                                        }}
                                      />
                                      <p>
                                        {res.free_space} free of{" "}
                                        {res.total_space}
                                      </p>
                                    </Col>
                                  </Row>
                                </Dropdown>
                              </Link>
                            </div>
                          </div>
                        );
                      }
                    })}
                  </Panel>
                </Collapse>
              </div>
            </div>
          </Col>
          <Col span={8}>
            <div className="card2">
              <div className="container">
                <div className="container-header">
                  <Space>
                    <h1>HELPS</h1>
                    <IoIosHelpCircle className="icon-help" />
                  </Space>
                </div>
                <>
                  <p>
                    File storage management is for file operation on Content
                    Server. User can put or delete file from content server hard
                    disk, or on User's USB drive. It support
                    <strong> NTFS, exFAT, and FAT32</strong>. Ext family and
                    other tree file system is not support for permission
                    problem.
                  </p>
                  <p>
                    On display, user will see content server and other{" "}
                    <strong>USB</strong>
                    plugged in with its total size and free space with our
                    beautiful interface.
                  </p>
                </>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default Storages;
