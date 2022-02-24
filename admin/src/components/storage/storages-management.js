import React, { useEffect, useState } from "react";
import {
  Row,
  Col,
  Layout,
  Button,
  message,
  Spin,
  Popover,
  Popconfirm,
} from "antd";
import { FiArrowLeft } from "react-icons/fi";
import { CaretRightOutlined } from "@ant-design/icons";
import { Link } from "react-router-dom";
import axios from "axios";
import CreateFolder from "./create-folder";
import StorageItem from "./storage-item";
import SendTO from "./send-to";

const { Content } = Layout;

const StoragesManagement = ({ match }) => {
  // -------state management ---------------
  const [visible, setVisible] = useState(false);
  const [loading, setLoading] = useState(false);
  const [dataStorage, setDataStorage] = useState({});
  const uuid = match.params.id;
  const [parent, setParent] = useState("/");
  const [selected, setSelected] = useState("");
  const [operation, setOperation] = useState("");
  const [sources, setSources] = useState([]);
  const [sourceUUID, setSourcesUUID] = useState("");
  const [showModal, setShowModal] = useState(false);

  // -------token ----------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  // -------- get data ----------

  async function fetchData() {
    await axios
      .get(`${baseUrl}/settings/storage/device/status/${uuid}`, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        setDataStorage(res.data);
        setLoading(false);
      })
      .catch((err) => console.log(err));
  }

  useEffect(() => {
    fetchData();
  }, []);

  // -------delete dir---------------

  const deleteDir = () => {
    axios
      .delete(`${baseUrl}/settings/storage/device/deletion`, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
        data: {
          selected_filedir: [`${selected}`],
          drive_partuuid: uuid,
        },
      })

      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          message.success("Successful!");
          fetchData();
          setLoading(false);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        console.log(err);
        message.error("Operation Failed! ");
      });
  };

  // ============= handle cut =============

  const handleCut = () => {
    setOperation("move");
    setSources(`${selected}`);
    setSourcesUUID(`${uuid}`);
  };

  // ==============handle Copy ==================

  const handleCopy = () => {
    setOperation("copy");
    setSources(`${selected}`);
    setSourcesUUID(`${uuid}`);
  };

  // ---------handle Paste  ---------

  const handlePaste = () => {
    const inputData = {
      operation: operation,
      source_uuid: sourceUUID,
      source_items: [`${sources}`],
      destination_uuid: uuid,
      items_destination: selected,
    };
    axios
      .post(`${baseUrl}/settings/storage/device/copy_or_move`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          message.success("Successful!");
          setLoading(false);
          fetchData();
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        console.log(err);
        message.error("Operation Failed! ");
      });
  };

  // ============handle unmount ============

  const handleUnmount = () => {
    const unmountData = {
      drive_partuuid: uuid,
    };
    axios
      .post(`${baseUrl}/settings/storage/device/unmount`, unmountData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          message.success("Successful!");
          setLoading(false);
          fetchData();
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        console.log(err);
        message.error("Operation Failed! ");
      });
  };

  const showCreateFoder = () => {
    setVisible(true);
  };
  const handleOk = () => {
    setVisible(false);
    setShowModal(false);
  };
  const handleCancel = () => {
    setVisible(false);
    setShowModal(false);
  };

  const showSendTo = () => {
    setShowModal(true);
  };

  if (loading) {
    return (
      <div className="spin">
        <Spin />
      </div>
    );
  }

  const OperationButtonFlash = () => {
    return (
      <React.Fragment>
        <div className="btn-options-storages">
          {selected ? (
            <div className="btn-options-storages2">
              <Button
                type="primary"
                className="button-update2"
                onClick={handleCopy}
              >
                Copy
              </Button>
              <Button
                type="primary"
                className="button-update2"
                onClick={handlePaste}
              >
                Paste
              </Button>
              <Button
                type="primary"
                className="button-update2"
                onClick={handleCut}
              >
                Cut
              </Button>
              <Popover
                content={contents}
                placement="rightTop"
                title={null}
                trigger="click"
                // visible={showModal}
                onVisibleChange={showSendTo}
              >
                <Button type="primary" className="button-update2">
                  Send to
                </Button>
              </Popover>
              <Button type="primary" className="button-update2">
                Delete
              </Button>
            </div>
          ) : (
            <div className="btn-options-storages2">
              <Button type="primary" className="button-unselected">
                Copy
              </Button>
              <Button type="primary" className="button-unselected">
                Paste
              </Button>
              <Button type="primary" className="button-unselected">
                Cut
              </Button>

              <Button type="primary" className="button-unselected">
                Send to
              </Button>

              <Button type="primary" className="button-unselected">
                Delete
              </Button>
            </div>
          )}
          <Button
            type="primary"
            className="button-update2"
            onClick={showCreateFoder}
          >
            New Folder
          </Button>
          <Button
            type="primary"
            className="button-update2"
            onClick={handleUnmount}
          >
            Safely Remove
          </Button>
        </div>
      </React.Fragment>
    );
  };

  const OperationButtonLocal = () => {
    return (
      <React.Fragment>
        <div className="btn-options-storages">
          {selected ? (
            <div className="btn-options-storages2">
              <Button
                type="primary"
                className="button-update2"
                onClick={handleCopy}
              >
                Copy
              </Button>
              <Button
                type="primary"
                className="button-update2"
                onClick={handlePaste}
              >
                Paste
              </Button>
              <Button
                type="primary"
                className="button-update2"
                onClick={handleCut}
              >
                Cut
              </Button>
              <Popover
                content={contents}
                placement="rightTop"
                title={null}
                trigger="click"
                // visible={showModal}
                onVisibleChange={showSendTo}
              >
                <Button type="primary" className="button-update2">
                  Send to
                </Button>
              </Popover>
              <Popconfirm
                title="Are you sure to delete it?"
                placement="rightTop"
                okText="Yes"
                cancelText="No"
                onConfirm={deleteDir}
                onCancel={handleCancel}
              >
                <Button type="primary" className="button-update2">
                  Delete
                </Button>
              </Popconfirm>
            </div>
          ) : (
            <div className="btn-options-storages2">
              <Button type="primary" className="button-unselected">
                Copy
              </Button>
              <Button type="primary" className="button-unselected">
                Paste
              </Button>
              <Button type="primary" className="button-unselected">
                Cut
              </Button>

              <Button type="primary" className="button-unselected">
                Send to
              </Button>

              <Button type="primary" className="button-unselected">
                Delete
              </Button>
            </div>
          )}
          <Button
            type="primary"
            className="button-update2"
            onClick={showCreateFoder}
          >
            New Folder
          </Button>
        </div>
      </React.Fragment>
    );
  };

  const contents = (
    <React.Fragment>
      <SendTO selected={selected} uuid={uuid} fetchData={fetchData} />
    </React.Fragment>
  );

  return (
    <React.Fragment>
      <CreateFolder
        visible={visible}
        handleCancel={handleCancel}
        handleOk={handleOk}
        uuid={uuid}
        selected={selected}
        fetchData={fetchData}
      />
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card">
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
                    {dataStorage.name}
                  </div>
                </div>
                {dataStorage.name !== "Local Content Storage" ? (
                  <OperationButtonFlash />
                ) : (
                  <OperationButtonLocal />
                )}
                <hr />
                <div className="local-data-container">
                  <div className="header-file-manager">
                    <Row getItem={12}>
                      <Col span={14}>
                        <p>Name</p>
                      </Col>
                      <Col span={8}>Date</Col>
                      <Col span={2}>
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
            </div>
          </Col>
          <Col span={8}>
            <div className="card2">
              <div className="container">
                <div className="container-header">
                  <h1>Desciptions</h1>
                </div>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default StoragesManagement;
