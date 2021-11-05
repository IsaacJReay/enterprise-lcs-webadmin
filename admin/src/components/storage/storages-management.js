import React, { useEffect, useState } from "react";
import { Row, Col, Layout, Button } from "antd";
import { FiArrowLeft } from "react-icons/fi";
import { CaretRightOutlined } from "@ant-design/icons";
import { Link } from "react-router-dom";
import axios from "axios";
import CreateFolder from "./create-folder";
import StorageItem from "./storage-item";

const { Content } = Layout;

const StoragesManagement = ({ match }) => {
  // -------state management ---------------
  const [visible, setVisible] = useState(false);
  const [loading, setLoading] = useState(false);
  const [dataStorage, setDataStorage] = useState({});
  const uuid = match.params.id;

  const [parent, setParent] = useState("/");
  const [selected, setSelected] = useState("");
  useEffect(() => {
    console.log("Selected: " + selected);
  }, [selected]);

  // -------token ----------

  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  // -------- get data ----------

  useEffect(async () => {
    setLoading(true);
    let data = await axios
      .get(
        `http://10.42.0.188:8080/private/api/settings/storage/device/status/${uuid}`,
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
        }
      )
      .then((res) => {
        return res.data;
      })
      .catch((err) => console.log(err));
    setDataStorage({ ...data });
    setLoading(false);
  }, []);

  const showCreateFoder = () => {
    setVisible(true);
  };
  const handleOk = () => {
    setVisible(false);
  };
  const handleCancel = () => {
    setVisible(false);
  };

  return (
    <React.Fragment>
      <CreateFolder
        visible={visible}
        handleCancel={handleCancel}
        handleOk={handleOk}
      />
      <Content>
        <Row gutter={[32, 32]}>
          <Col span={16}>
            <div className="container">
              <div className="container-header">
                <h1>File Storages</h1>
              </div>
              <hr />
              <div className="header-storages">
                <Button type="primary" shape="circle" size="large">
                  <Link to="/storages">
                    <FiArrowLeft className="back-btn" />
                  </Link>
                </Button>
                <div className="header-storage-2">
                  <CaretRightOutlined />
                  {dataStorage.drive_label}
                </div>
              </div>
              {dataStorage.drive_label === "Local Content Storage" ? (
                <div className="btn-options-storages">
                  <Button type="primary" className="button-update">
                    Copy
                  </Button>
                  <Button type="primary" className="button-update">
                    Past
                  </Button>
                  <Button type="primary" className="button-update">
                    Move
                  </Button>
                  <Button type="primary" className="button-update">
                    Delete
                  </Button>
                  <Button
                    type="primary"
                    className="button-update"
                    onClick={showCreateFoder}
                  >
                    New Folder
                  </Button>
                  <Button type="primary" className="button-update">
                    Mount R/W
                  </Button>
                  <Button type="primary" className="button-update">
                    Safely Remove
                  </Button>
                </div>
              ) : (
                <div className="btn-options-storages">
                  <Button type="primary" className="button-update">
                    Copy
                  </Button>
                  <Button type="primary" className="button-update">
                    Past
                  </Button>
                  <Button type="primary" className="button-update">
                    Move
                  </Button>
                  <Button type="primary" className="button-update">
                    Delete
                  </Button>
                  <Button
                    type="primary"
                    className="button-update"
                    onClick={showCreateFoder}
                  >
                    New Folder
                  </Button>
                </div>
              )}
              <hr />
              <div className="local-data-container">
                <div className="header-file-manager">
                  <Row getItem={12}>
                    <Col span={20}>
                      {" "}
                      <p>Name</p>
                    </Col>

                    <Col span={4}>
                      <p>Size</p>
                    </Col>
                  </Row>
                </div>
                {!loading &&
                  dataStorage.children &&
                  JSON.parse(JSON.stringify(dataStorage)).children.map(
                    (item) => (
                      <StorageItem
                        data={item}
                        parent={parent}
                        setParent={setParent}
                        setSelected={setSelected}
                        selected={selected}
                      />
                    )
                  )}
              </div>
            </div>
          </Col>
          <Col span={8}>
            <div className="container">
              <div className="container-header">
                <h1>Desciptions</h1>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default StoragesManagement;
