import React, { useEffect, useState } from "react";
import {
  Row,
  Col,
  Layout,
  Button,
  message,
  Spin,
  Popover,
  Space,
  Modal,
} from "antd";
import { FiArrowLeft, FiFolderPlus, FiSend, FiCopy } from "react-icons/fi";
import {
  CaretRightOutlined,
  ExclamationCircleOutlined,
} from "@ant-design/icons";
import { Link, useParams } from "react-router-dom";
import axios from "axios";
import CreateFolder from "./create-folder";
import StorageItem from "./storage-item";
import SendTO from "./send-to";
import { IoIosHelpCircle } from "react-icons/io";
import { MdOutlineContentPaste, MdDelete } from "react-icons/md";
import { BsFolderX } from "react-icons/bs";
import Cookies from "js-cookie";

const { Content } = Layout;

const StoragesManagement = () => {
  // -------state management ---------------
  const [visible, setVisible] = useState(false);
  const [loading, setLoading] = useState(false);
  const [dataStorage, setDataStorage] = useState({});
  const { id } = useParams();
  const uuid = id;
  const [parent, setParent] = useState("/");
  const [selected, setSelected] = useState("");
  const [operation, setOperation] = useState("");
  const [sources, setSources] = useState([]);
  const [sourceUUID, setSourcesUUID] = useState("");
  const [showModal, setShowModal] = useState(false);

  // -------token ----------
  const baseUrl = process.env.REACT_APP_API_URL;
  // const getToken = localStorage.getItem("token");
  const getToken = Cookies.get("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  // -------- get data ----------

  function fetchData() {
    axios
      .get(`${baseUrl}/settings/storage/device/status/${uuid}`, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        setDataStorage(res.data);
      })
      .catch((err) => console.log(err));
  }

  useEffect(() => {
    fetchData();
  }, []);

  // ============= handle cut =============

  const handleCut = () => {
    setOperation("move");
    setSources(`${selected}`);
    setSourcesUUID(`${uuid}`);
    message.success("Moved");
  };

  // ==============handle Copy ==================

  const handleCopy = () => {
    setOperation("copy");
    setSources(`${selected}`);
    setSourcesUUID(`${uuid}`);
    message.success("Copied");
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
        if ((res.statusCode = 200)) {
          setLoading(true);
          message.success("Successful!");
          setLoading(false);
          fetchData();
        } else {
          message.error("Operation Failed! ");
        }
      })

      .catch((err) => {
        console.log(err);
      });
  };
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
        if ((res.statusCode = 200)) {
          message.success("Successful!");
          fetchData();
        } else {
          message.error("Operation Failed! ");
        }
      })
      .catch((err) => {
        console.log(err);
      });
  };

  const showCreateFolder = () => {
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

  const OperationButtonLocal = () => {
    return (
      <div className="btn-options-storages">
        <Row gutter={6} justify="end">
          {selected && (
            <>
              <Col>
                <Popover title={null} content="Send Folder">
                  <FiSend className="create-folder" onClick={showSendTo} />
                </Popover>
              </Col>
              <Col>
                <Popover title={null} content="Move">
                  <BsFolderX className="create-folder" onClick={handleCut} />
                </Popover>
              </Col>
              <Col>
                <Popover title={null} content="Copy">
                  <FiCopy className="create-folder" onClick={handleCopy} />
                </Popover>
              </Col>
              <Col>
                <Popover title={null} content="Paste">
                  <MdOutlineContentPaste
                    className="create-folder"
                    onClick={handlePaste}
                  />
                </Popover>
              </Col>
              <Col>
                <Popover title={null} content="Delete">
                  <MdDelete
                    className="delete-folder"
                    onClick={() => {
                      Modal.confirm({
                        title: "Are you sure to delte it?",
                        icon: <ExclamationCircleOutlined />,
                        content: "Make sure you can lose your data!",
                        okText: "Delete",
                        cancelText: "Cancel",
                        onOk: deleteDir,
                      });
                    }}
                  />
                </Popover>
              </Col>
            </>
          )}
          {dataStorage && (
            <Col>
              <Popover title={null} content="New folder">
                <FiFolderPlus
                  className="create-folder"
                  onClick={showCreateFolder}
                />
              </Popover>
            </Col>
          )}
        </Row>
      </div>
    );
  };

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
      <SendTO
        selected={selected}
        uuid={uuid}
        fetchData={fetchData}
        showModal={showModal}
        handleCancel={handleCancel}
        handleOk={handleOk}
      />
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card">
              <div className="container">
                <div className="container-header">
                  <h1>STORAGE MANAGEMENT</h1>
                </div>
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

                <OperationButtonLocal />

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
                          fetchData={fetchData}
                          data={item}
                          parent={parent}
                          setParent={setParent}
                          setSelected={setSelected}
                          selected={selected}
                          setSources={setSources}
                          setSourcesUUID={setSourcesUUID}
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
                  <Space>
                    <h1>HELPS</h1>
                    <IoIosHelpCircle className="icon-help" />
                  </Space>
                </div>
                <p>
                  There are 5 operations open to user for control their file on
                  their USB, which includes{" "}
                  <strong>
                    {" "}
                    Copy, Move, Delete, Create Folder, and Safely unmount their
                    USB drive
                  </strong>
                  .
                </p>
                <p>
                  Similar operation are also available for content server, but
                  unmount is not allowed.
                </p>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default StoragesManagement;
