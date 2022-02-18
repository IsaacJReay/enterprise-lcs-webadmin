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

  // -------delete dir---------------

  const deleteDir = () => {
    axios
      .delete(
        "http://10.42.0.188:8080/private/api/settings/storage/device/deletion",
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
          data: {
            selected_filedir: [`${selected}`],
            drive_partuuid: uuid,
          },
        }
      )

      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          message.success("Successful!");
          setLoading(false);
          window.location.reload();
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
      .post(
        "http://10.42.0.188:8080/private/api/settings/storage/device/copy_or_move",
        inputData,
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
        }
      )
      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          message.success("Successful!");
          setLoading(false);
          window.location.reload();
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
      .post(
        "http://10.42.0.188:8080/private/api/settings/storage/device/unmount",
        unmountData,
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
        }
      )
      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          message.success("Successful!");
          setLoading(false);
          window.location.reload();
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

  const OperationButton = () => {
    return (
      <React.Fragment>
        <div className="btn-options-storages">
          <Button type="primary" className="button-update" onClick={handleCopy}>
            Copy
          </Button>
          <Button
            type="primary"
            className="button-update"
            onClick={handlePaste}
          >
            Paste
          </Button>
          <Button type="primary" className="button-update" onClick={handleCut}>
            Cut
          </Button>
          <Popover
            content={contents}
            placement="rightTop"
            title={null}
            trigger="click"
            visible={showModal}
            onVisibleChange={showSendTo}
          >
            <Button type="primary" className="button-update">
              Send to
            </Button>
          </Popover>
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
          <Button
            type="primary"
            className="button-update"
            onClick={handleUnmount}
          >
            Safely Remove
          </Button>
        </div>
      </React.Fragment>
    );
  };

  const contents = (
    <React.Fragment>
      <SendTO selected={selected} uuid={uuid} />
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
                  <OperationButton />
                ) : (
                  <div className="btn-options-storages">
                    <Button
                      type="primary"
                      className="button-update"
                      onClick={handleCopy}
                    >
                      Copy
                    </Button>
                    <Button
                      type="primary"
                      className="button-update"
                      onClick={handlePaste}
                    >
                      Paste
                    </Button>
                    <Button
                      type="primary"
                      className="button-update"
                      onClick={handleCut}
                    >
                      Cut
                    </Button>
                    <Popover
                      content={contents}
                      placement="rightTop"
                      title={null}
                      trigger="click"
                      visible={showModal}
                      onVisibleChange={showSendTo}
                    >
                      <Button type="primary" className="button-update">
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
                      <Button type="primary" className="button-update">
                        Delete
                      </Button>
                    </Popconfirm>
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
            <div className="card">
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
