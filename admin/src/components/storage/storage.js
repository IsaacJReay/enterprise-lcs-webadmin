import React, { useState, useEffect } from "react";
import { Layout, Col, Row, Collapse, Progress, Space } from "antd";
import { CaretRightOutlined } from "@ant-design/icons";
import myImage from "../../assets/images/Hard-Drive3.png";
import driver from "../../assets/images/Hard-Drive.png";
import { Link } from "react-router-dom";
import axios from "axios";
import { IoIosHelpCircle } from "react-icons/io";

const { Content } = Layout;
const { Panel } = Collapse;

const Storages = () => {
  // -------state management ---------------

  const [, setLoading] = useState(false);
  const [storages, setStorages] = useState([]);

  // -------token ----------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  useEffect(() => {
    setLoading(true);
    axios({
      method: "GET",
      url: `${baseUrl}/settings/storage/status`,
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setStorages(res.data);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }, []);

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
                                      {res.free_space} free of {res.total_space}
                                    </p>
                                  </Col>
                                </Row>
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
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default Storages;
