import React, { useState, useEffect } from "react";
import {
  Layout,
  Col,
  Row,
  Button,
  Form,
  Table,
  Select,
  Tag,
  Popover,
  Popconfirm,
  message,
  Collapse,
  Checkbox,
} from "antd";
import { FiEdit } from "react-icons/fi";
import DNSRename from "./rename";
import { Link } from "react-router-dom";
import AddRecord from "./adding-record";
import { PlusOutlined, CaretRightOutlined } from "@ant-design/icons";
import axios from "axios";

const { Content } = Layout;
const { Option } = Select;
const { Panel } = Collapse;

const DNSManagement = ({ match }) => {
  const [visible, setVisible] = useState(false);
  const [records, setRecords] = useState(false);
  const [doid, setDoId] = useState({});
  const [doname, setDoname] = useState({});
  const [, setLoading] = useState(false);
  const [items, setItems] = useState({});
  const [subdomain, setSubdomain] = useState([]);
  const [form] = Form.useForm();

  const key = match.params.id;

  // ------token ------

  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  // ----------get data -------------

  useEffect(() => {
    async function fetchData() {
      await axios
        .get(
          `http://10.42.0.188:8080/private/api/settings/dns/zone_records/status/${key}`,
          {
            headers: {
              "content-type": "application/json",
              ...auth,
            },
          }
        )
        .then((res) => {
          setLoading(true);
          setItems(res.data);
          setSubdomain(res.data.record_table);
          const { status } = res.data;
          form.setFieldsValue({
            status: status,
          });
          setTimeout(() => {
            setLoading(false);
          }, 1000);
        })
        .catch((err) => console.log(err));
    }
    fetchData();
  }, [items]);

  //  --------delete record ----------

  const handleDelete = async (id) => {
    await axios
      .delete(
        "http://10.42.0.188:8080/private/api/settings/dns/zone_record/deletion",

        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
          data: { id: id, foreign_key: `${match.params.id}` },
        }
      )
      .then((res) => {
        if (res.data.operation_status === "Success") {
          setTimeout(() => {
            message.success("Successful!");
          }, 1000);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        console.log(err);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        message.error("Operation Failed! ");
      });
  };

  // ---------- hosting domain ----------
  const handleApply = async (data) => {
    const inputData = {
      id: key,
      status: data.status,
    };
    await axios
      .put(
        "http://10.42.0.188:8080/private/api/settings/dns/status/update",
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
          setTimeout(() => {
            message.success("Successful!");
          }, 1000);
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

  const showModalRename = () => {
    setVisible(true);
    setDoId(`${match.params.id}`);
    setDoname(items.domain_name);
  };

  const handleCancel = () => {
    setVisible(false);
    setRecords(false);
  };

  const handleOk = () => {
    setVisible(false);
    setRecords(false);
  };

  const showAddRecords = () => {
    setRecords(true);
    setDoId(`${match.params.id}`);
  };

  const columns = [
    {
      title: "N0",
      dataIndex: "id",
      key: "id",
      width: "10%",
    },
    {
      title: "Sub Domain",
      dataIndex: "subdomain_name",
      key: "subdomain_name",
      editable: true,
      width: "20%",
    },
    {
      title: "Address",
      dataIndex: "address",
      key: "address",
      editable: true,
      width: "20%",
    },
    {
      title: "Type",
      dataIndex: "dns_type",
      key: "dns_type",
      editable: true,
      render: (dns_type) => {
        return (
          <React.Fragment>
            <Select
              disabled
              defaultValue={dns_type}
              size="large"
              className="select-option"
            >
              <Option value="A">A</Option>
              <Option value="MX 10">MX 10</Option>
              <Option value="CNAME">CNAME</Option>
            </Select>
          </React.Fragment>
        );
      },
    },
    {
      title: "Actions",
      dataIndex: "id",
      width: "20%",
      render: (id, data) => {
        const name = data.subdomain_name;
        return (
          <React.Fragment>
            <Popconfirm
              placement="top"
              title={
                <span>
                  Are you sure to delete <b>{name} </b>?
                </span>
              }
              okText="Yes"
              cancelText="No"
              onConfirm={() => handleDelete(id)}
            >
              <Tag color="error" style={{ cursor: "pointer" }}>
                Delete
              </Tag>
            </Popconfirm>
          </React.Fragment>
        );
      },
    },
  ];

  const HostingDomain = () => {
    return (
      <React.Fragment>
        <div className="card">
          <div className="container">
            <div className="container-header">
              <h1>Domain Setting</h1>
            </div>
            <hr />

            <Collapse
              bordered={false}
              expandIcon={({ isActive }) => (
                <CaretRightOutlined rotate={isActive ? 90 : 0} />
              )}
            >
              <Panel header="Hosting Domain!" key="1">
                <Form onFinish={handleApply} form={form}>
                  <Form.Item name="status" valuePropName="checked">
                    <Checkbox style={{ color: "red ", paddingLeft: "20px" }}>
                      Hosting Domain!
                    </Checkbox>
                  </Form.Item>
                  <Form.Item>
                    <Button
                      type="primary"
                      className="button-apply2"
                      htmlType="submit"
                    >
                      Apply
                    </Button>
                  </Form.Item>
                </Form>
              </Panel>
            </Collapse>
          </div>
        </div>
      </React.Fragment>
    );
  };

  return (
    <React.Fragment>
      <Content>
        <DNSRename
          visible={visible}
          handleCancel={handleCancel}
          handleOk={handleOk}
          doid={doid}
          doname={doname}
        />
        <AddRecord
          records={records}
          handleCancel={handleCancel}
          handleOk={handleOk}
          doid={doid}
        />
        <Row gutter={12}>
          <Col span={16}>
            <Row gutter={[12, 12]}>
              <Col span={24}>
                <div>
                  <HostingDomain />
                </div>
              </Col>
              <Col>
                <Form>
                  <div className="card">
                    <div className="container">
                      <div className="container-header">
                        <h1>DNS Setting</h1>
                      </div>
                      <hr />

                      <div className="dns-desc-container">
                        <Form.Item label="Domain Name">
                          <Row gutter={[6, 0]}>
                            <Col>
                              <p className="domain_name">{items.domain_name}</p>
                            </Col>
                            <Col>
                              <Popover
                                title={null}
                                content="Rename"
                                placement="topLeft"
                              >
                                <FiEdit onClick={showModalRename} />
                              </Popover>
                            </Col>
                          </Row>
                        </Form.Item>
                      </div>

                      <div className="dns-desc-container">
                        <Form.Item>
                          <Button
                            type="primary"
                            className="button-update"
                            onClick={showAddRecords}
                          >
                            <PlusOutlined />
                            Add Record
                          </Button>
                        </Form.Item>
                        <Table
                          columns={columns}
                          dataSource={subdomain}
                          pagination={false}
                          scroll={{ y: 200 }}
                        />
                      </div>

                      <div className="container-buttons">
                        <Form.Item>
                          <Link to="/dns">
                            <Button
                              type="primary"
                              htmlType="button"
                              className="button-apply2"
                              size="large"
                            >
                              Back
                            </Button>
                          </Link>
                        </Form.Item>
                      </div>
                    </div>
                  </div>
                </Form>
              </Col>
            </Row>
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

export default DNSManagement;
